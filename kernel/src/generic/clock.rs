use alloc::boxed::Box;
use spin::mutex::Mutex;

pub trait ClockSource: Send {
    fn name(&self) -> &'static str;

    /// Gets the elapsed nanoseconds since initialization of this timer.
    fn get_elapsed(&self) -> usize;

    /// Sets the elapsed nanoseconds to start counting at.
    fn set_elapsed(&self, elapsed: usize);
}

pub struct Clock {
    current: Option<Box<dyn ClockSource>>,
}

static CLOCK: Mutex<Clock> = Mutex::new(Clock { current: None });

impl Clock {
    /// Gets the elapsed nanoseconds since initialization of this timer.
    pub fn get_elapsed() -> usize {
        let guard = CLOCK.lock();
        match &guard.current {
            Some(x) => x.get_elapsed(),
            None => 0,
        }
    }

    /// Switches to a new ClockSource.
    pub fn switch(new: Box<dyn ClockSource>) {
        // Copy over the current clock value.
        let old_elapsed = Clock::get_elapsed();
        new.set_elapsed(old_elapsed);

        print!("Switching to new clock source \"{}\"", new.name());
        CLOCK.lock().current = Some(new);
    }
}
