#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
		let current_time = $crate::generic::clock::get_elapsed();
		_ = writer.write_fmt(format_args!("[{:5}.{:06}] ", current_time / 1000000000, (current_time / 1000) % 1000000));
		_ = writer.write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
		let current_time = $crate::generic::clock::get_elapsed();
		_ = writer.write_fmt(format_args!("[{:5}.{:06}] warn: ", current_time / 1000000000, (current_time / 1000) % 1000000));
		_ = writer.write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
		let current_time = $crate::generic::clock::get_elapsed();
		_ = writer.write_fmt(format_args!("[{:5}.{:06}] error: ", current_time / 1000000000, (current_time / 1000) % 1000000));
		_ = writer.write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
		let current_time = $crate::generic::clock::get_elapsed();
		_ = writer.write_fmt(format_args!("[{:5}.{:06}] ", current_time / 1000000000, (current_time / 1000) % 1000000));
		_ = writer.write_fmt(format_args!("{:#?}\n", $($arg)*));
    });
}

#[macro_export]
macro_rules! assert_size {
    ($x:ty, $xs:expr) => {
        const _: fn() = || {
            let _ = $crate::core::mem::transmute::<$x, [u8; $xs]>;
        };
    };
}
