use super::asm;
use crate::generic::clock::{self, ClockError, ClockSource};
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
        return (asm::rdtsc() * 1_000_000_000 / TSC_FREQUENCY.load(Ordering::Relaxed)) as usize;
    }

    fn setup(&mut self) -> Result<(), ClockError> {
        // Check if we have the TSC info leaf.
        let cpuid = match asm::cpuid(0x8000_0000, 0).eax >= 0x15 {
            true => Some(asm::cpuid(0x15, 0)),
            false => None,
        };

        // First, always try using another known good clock to calibrate.
        let freq = if clock::has_clock() {
            print!("tsc: Calibrating using exisiting clock.\n");
            let t1 = asm::rdtsc();
            clock::wait_ns(1_000_000_000);
            let t2 = asm::rdtsc();

            t2 - t1
        }
        // If we have no timer (yet), the only way we can calibrate the TSC is if CPUID gives us the frequency.
        // On a normal system, this should usually never be called and is a last resort
        // since at this point we have at least the HPET timer.
        // TODO: Also check invariant bit in CPUID.
        else if let Some(c) = cpuid {
            print!("tsc: Calibrating using CPUID 0x15.\n");
            if c.ecx != 0 && c.ebx != 0 && c.eax != 0 {
                c.ecx as u64 * c.ebx as u64 / c.eax as u64
            } else {
                return Err(ClockError::InvalidConfiguration);
            }
        }
        // We tried.
        else {
            return Err(ClockError::UnableToCalibrate);
        };

        print!("tsc: Timer frequency is {freq} Hz.\n");
        TSC_FREQUENCY.store(freq, Ordering::Relaxed);

        return Ok(());
    }
}
