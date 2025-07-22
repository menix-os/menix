use uacpi_sys::{UACPI_STATUS_OK, uacpi_table, uacpi_table_find_by_signature};

pub fn parse_mcfg() {
    unsafe {
        let mut table = uacpi_table::default();
        let status = uacpi_table_find_by_signature(c"MCFG".as_ptr(), &raw mut table);
        if status != UACPI_STATUS_OK {
            return;
        }

        // We have an MCFG, so reinitialize the PciAccess callbacks.
        todo!();
        //crate::system::pci::config::ACCESS.init();

        let mcfg_ptr = table.__bindgen_anon_1.ptr as *const uacpi_sys::acpi_mcfg;
        let mcfg = mcfg_ptr.read_unaligned();
    };
}
