use alloc::boxed::Box;
use spin::mutex::Mutex;

pub trait ClockSource: Send {
    fn name(&self) -> &'static str;

    /// Gets the elapsed nanoseconds since initialization of this timer.
    fn get_elapsed_ns(&self) -> usize;

    /// Sets the elapsed nanoseconds to start counting at.
    fn reset(&self);
}

pub struct Clock {
    /// The active clock source.
    current: Option<Box<dyn ClockSource>>,
    /// An offset to add to the read counter.
    counter_base: usize,
}

impl Clock {
    /// Gets the elapsed nanoseconds since initialization of this timer.
    pub fn get_elapsed() -> usize {
        let guard = CLOCK.lock();
        match &guard.current {
            Some(x) => x.get_elapsed_ns(),
            None => 0,
        }
    }

    /// Switches to a new ClockSource.
    pub fn switch(new_source: Box<dyn ClockSource>) {
        let mut clock = CLOCK.lock();
        print!("Switching to clock source \"{}\"", new_source.name());

        // Save the current counter.
        clock.counter_base = Clock::get_elapsed();
        // Reset the new counter.
        new_source.reset();
        // Save the new counter.
        clock.current = Some(new_source);
    }
}

static CLOCK: Mutex<Clock> = Mutex::new(Clock {
    current: None,
    counter_base: 0,
});
