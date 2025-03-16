use super::asm;
use crate::generic::clock::{self, ClockSource};
use core::sync::atomic::{AtomicU64, Ordering};

pub struct TscClock;

static TSC_FREQUENCY: AtomicU64 = AtomicU64::new(0);

impl ClockSource for TscClock {
    fn name(&self) -> &'static str {
        "tsc"
    }

    fn reset(&mut self) {
        // The TSC can't be reset.
    }

    fn get_priority(&self) -> u8 {
        // Prefer the TSC over other timers.
        return 255;
    }

    fn get_elapsed_ns(&self) -> usize {
        return (asm::rdtsc() * 1000 / TSC_FREQUENCY.load(Ordering::Relaxed)) as usize;
    }
}

pub fn setup() -> bool {
    let c = asm::cpuid(0x15, 0);

    let freq = if clock::has_clock() {
        // Wait 1 second to calibrate
        let t1 = asm::rdtsc();
        clock::wait_ns(1_000_000_000);
        let t2 = asm::rdtsc();

        t2 - t1
    } else if c.ecx != 0 && c.ebx != 0 {
        c.ecx as u64 * c.ebx as u64 / c.eax as u64
    } else {
        warn!("tsc: Calibration failed.\n");
        return false;
    };

    print!("tsc: Timer frequency is {freq} Hz.\n");
    TSC_FREQUENCY.store(freq, Ordering::Relaxed);
    return true;
}
