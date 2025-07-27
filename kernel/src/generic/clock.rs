//! Global timer management.
// TODO: Try to get rid of some locks.

use super::util::spin_mutex::SpinMutex;
use alloc::boxed::Box;

#[initgraph::task(name = "generic.clock")]
pub fn CLOCK_STAGE() {}

pub trait ClockSource: Send {
    fn name(&self) -> &'static str;

    /// A priority of a clock source. A high value equals a high priority.
    fn get_priority(&self) -> u8;

    /// Sets the elapsed nanoseconds to start counting at.
    fn reset(&mut self);

    /// Gets the elapsed nanoseconds since initialization of this timer.
    fn get_elapsed_ns(&self) -> usize;
}

#[derive(Debug, PartialEq)]
pub enum ClockError {
    /// The clock source has a lesser priority.
    LowerPriority,
    /// The clock source is unavailable.
    Unavailable,
    /// The clock source is not sane.
    InvalidConfiguration,
    /// The clock source could not be calibrated.
    UnableToSetup,
}

/// Gets the elapsed nanoseconds since initialization of this timer.
pub fn get_elapsed() -> usize {
    let guard = CLOCK.lock();
    match &guard.current {
        Some(x) => x.get_elapsed_ns() + guard.counter_base,
        None => 0,
    }
}

/// Switches to a new clock source if it is of higher priority.
pub fn switch(mut new_source: Box<dyn ClockSource>) -> Result<(), ClockError> {
    // Determine if we should make the switch.
    if let Some(x) = &CLOCK.lock().current {
        let prio = x.get_priority();
        if new_source.get_priority() > prio {
            Ok(())
        } else {
            Err(ClockError::LowerPriority)
        }
    } else {
        Ok(())
    }?;

    log!("Switching to clock source \"{}\"", new_source.name());

    // Save the current counter.
    let elapsed = get_elapsed();
    let mut clock = CLOCK.lock();
    clock.counter_base = elapsed;

    new_source.reset();
    clock.current = Some(new_source);
    return Ok(());
}

pub fn has_clock() -> bool {
    return CLOCK.lock().current.is_some();
}

/// Blocking wait for a given amount of nanoseconds.
pub fn block_ns(time: usize) -> Result<(), ClockError> {
    if CLOCK.lock().current.is_none() {
        error!(
            "Unable to sleep for {} nanoseconds. No clock source available, this would block forever!",
            time
        );
        return Err(ClockError::Unavailable);
    }

    let target = get_elapsed() + time;
    while get_elapsed() < target {}
    return Ok(());
}

struct Clock {
    /// The active clock source.
    current: Option<Box<dyn ClockSource>>,
    /// An offset to add to the read counter.
    counter_base: usize,
}

static CLOCK: SpinMutex<Clock> = SpinMutex::new(Clock {
    current: None,
    counter_base: 0,
});
