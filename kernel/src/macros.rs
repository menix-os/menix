#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::KERNEL_LOGGER.lock();

		let current_time = $crate::generic::clock::Clock::get_elapsed();
		writer.write_fmt(format_args!("[{:5}.{:06}] ", current_time / 1000000000, (current_time / 1000) % 1000000)).unwrap();

		writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::KERNEL_LOGGER.lock();

		let current_time = $crate::generic::clock::Clock::get_elapsed();
		writer.write_fmt(format_args!("[{:5}.{:06}] ", current_time / 1000000000, (current_time / 1000) % 1000000)).unwrap();

		writer.write_fmt(format_args!("{:#?}\n", $($arg)*)).unwrap();
    });
}
