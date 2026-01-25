use crate::{
    irq::{Polarity, TriggerMode},
    util::mutex::spin::SpinMutex,
};
use uacpi_sys::{UACPI_STATUS_OK, uacpi_table, uacpi_table_find_by_signature, uacpi_table_unref};

static SOURCE_OVERRIDES: SpinMutex<[Option<(u32, TriggerMode, Polarity)>; 16]> =
    SpinMutex::new([const { None }; _]);

#[initgraph::task(
    name = "system.acpi.madt",
    depends = [crate::system::acpi::INIT_STAGE],
)]
fn IOAPIC_STAGE() {
    unsafe {
        let mut table = uacpi_table::default();
        let status = uacpi_table_find_by_signature(c"APIC".as_ptr(), &raw mut table);
        if status != UACPI_STATUS_OK {
            return;
        }

        let madt_ptr = table.__bindgen_anon_1.ptr as *const uacpi_sys::acpi_madt;
        let madt = madt_ptr.read_unaligned();

        let mut offset = size_of::<uacpi_sys::acpi_madt>();

        while offset < madt.hdr.length as usize {
            let entry_ptr = madt_ptr.byte_add(offset) as *const uacpi_sys::acpi_entry_hdr;
            let entry = entry_ptr.read_unaligned();

            if entry.type_ as u32 == uacpi_sys::ACPI_MADT_ENTRY_TYPE_INTERRUPT_SOURCE_OVERRIDE {
                let entry = (entry_ptr as *const uacpi_sys::acpi_madt_interrupt_source_override)
                    .read_unaligned();

                assert!(entry.source < 16);

                let trigger = match entry.flags as u32 & uacpi_sys::ACPI_MADT_TRIGGERING_MASK {
                    uacpi_sys::ACPI_MADT_TRIGGERING_EDGE => TriggerMode::Edge,
                    uacpi_sys::ACPI_MADT_TRIGGERING_LEVEL => TriggerMode::Level,
                    _ => TriggerMode::Edge,
                };
                let polarity = match entry.flags as u32 & uacpi_sys::ACPI_MADT_POLARITY_MASK {
                    uacpi_sys::ACPI_MADT_POLARITY_ACTIVE_HIGH => Polarity::High,
                    uacpi_sys::ACPI_MADT_POLARITY_ACTIVE_LOW => Polarity::Low,
                    _ => Polarity::High,
                };

                let gsi = entry.gsi;
                log!(
                    "ISA override: GSI {}, Trigger mode: {:?}, Polarity: {:?}",
                    gsi,
                    trigger,
                    polarity
                );
                *SOURCE_OVERRIDES
                    .lock()
                    .get_mut(entry.source as usize)
                    .unwrap() = Some((entry.gsi, trigger, polarity));
            }

            offset += entry.length as usize;
        }

        uacpi_table_unref(&mut table);
    }
}
