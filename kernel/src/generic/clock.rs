use alloc::boxed::Box;
use spin::mutex::Mutex;

pub trait ClockSource: Send {
    fn name(&self) -> &'static str;

    /// Sets the elapsed nanoseconds to start counting at.
    fn reset(&mut self);

    /// A priority of a clock source. A high value equals a high priority.
    fn get_priority(&self) -> u8;

    /// Gets the elapsed nanoseconds since initialization of this timer.
    fn get_elapsed_ns(&self) -> usize;
}

pub struct Clock {
    /// The active clock source.
    current: Option<Box<dyn ClockSource>>,
    /// An offset to add to the read counter.
    counter_base: usize,
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
pub fn switch(mut new_source: Box<dyn ClockSource>) {
    let clock = CLOCK.lock();
    match &clock.current {
        Some(x) => {
            let prio = x.get_priority();
            if new_source.get_priority() > prio {
                drop(clock);
                force_switch(new_source);
            }
        }
        None => {
            drop(clock);
            force_switch(new_source);
        }
    }
}

/// Forcefully switches to a new clock source.
fn force_switch(mut new_source: Box<dyn ClockSource>) {
    print!(
        "clock: Switching to clock source \"{}\"\n",
        new_source.name()
    );
    // Save the current counter.
    let elapsed = get_elapsed();
    let mut clock = CLOCK.lock();
    clock.counter_base = elapsed;

    new_source.reset();
    clock.current = Some(new_source);
}

pub fn has_clock() -> bool {
    return CLOCK.lock().current.is_some();
}

/// Blocking wait for a given amount of nanoseconds.
pub fn wait_ns(time: usize) {
    let target = get_elapsed() + time;
    while get_elapsed() < target {}
}

static CLOCK: Mutex<Clock> = Mutex::new(Clock {
    current: None,
    counter_base: 0,
});
