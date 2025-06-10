use super::asm;
use crate::{
    arch::x86_64::consts,
    generic::clock::{self, ClockError, ClockSource},
};
use core::sync::atomic::{AtomicU64, Ordering};

static TSC_FREQUENCY: AtomicU64 = AtomicU64::new(0);
static TSC_BASE: AtomicU64 = AtomicU64::new(0);

pub struct TscClock;
impl ClockSource for TscClock {
    fn name(&self) -> &'static str {
        "tsc"
    }

    fn reset(&mut self) {
        // The TSC can't be set manually, so we record whatever value it had when `reset` was called and subtract that.
        TSC_BASE.store(
            asm::rdtsc() * 1_000_000_000 / TSC_FREQUENCY.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
    }

    fn get_priority(&self) -> u8 {
        // Prefer the TSC over other timers.
        return 255;
    }

    // TODO: This wraps after like 5 seconds. Fix this, then reenable tsc as default.
    fn get_elapsed_ns(&self) -> usize {
        return (asm::rdtsc() * 1_000_000_000 / TSC_FREQUENCY.load(Ordering::Relaxed)
            - TSC_BASE.load(Ordering::Relaxed)) as usize;
    }
}

init_stage! {
    #[entails(crate::generic::clock::CLOCK_STAGE)]
    TSC_STAGE: "arch.x86_64.tsc" => init;
}

fn init() {
    // We need an invariant TSC.
    if asm::cpuid(1, 0).edx & consts::CPUID_1D_TSC == 0
        || asm::cpuid(0x8000_0007, 0).edx & (1 << 8) == 0
    {
        return;
    }

    // Check if we have the TSC info leaf.
    let cpuid = match asm::cpuid(0x8000_0000, 0).eax >= 0x15 {
        true => Some(asm::cpuid(0x15, 0)),
        false => None,
    };

    // First, always try using another known good clock to calibrate.
    let freq = if clock::has_clock() {
        log!("Calibrating using exisiting clock");

        // Wait for 10ms.
        let t1 = asm::rdtsc();
        clock::block_ns(10_000_000).unwrap();
        let t2 = asm::rdtsc();

        // We want the frequency in Hz.
        // TODO: This might be imprecise.
        (t2 - t1) * 100
    } else if let Some(c) = cpuid {
        // If we have no timer (yet), the only way we can calibrate the TSC is if CPUID gives us the frequency.
        // On a normal system, this should usually never be called and is a last resort
        // since at this point we have at least the HPET timer.
        log!("Calibrating using CPUID 0x15");
        if c.ecx != 0 && c.ebx != 0 && c.eax != 0 {
            c.ecx as u64 * c.ebx as u64 / c.eax as u64
        } else {
            return;
        }
    }
    // We tried.
    else {
        return;
    };

    log!("Timer frequency is {} MHz ({} Hz)", freq / 1_000_000, freq);
    TSC_FREQUENCY.store(freq, Ordering::Relaxed);
}
