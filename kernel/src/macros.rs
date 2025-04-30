#[macro_export]
macro_rules! static_assert {
    ($xs:expr) => {
        const _: () = assert!($xs, concat!("Assertion failed: \"", stringify!($xs), "\"!"));
    };
}

/// For regular per-CPU variables.
#[macro_export]
macro_rules! thread_local {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => {
        #[unsafe(link_section = ".percpu")]
        $vis static $name: $crate::generic::cpu::PerCpuData<$t> =
            $crate::generic::cpu::PerCpuData::new($init);
    };
}

#[macro_export]
macro_rules! print_inner {
    ($prefix:literal, $suffix:literal, $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
        let current_time = $crate::generic::clock::get_elapsed();
        _ = writer.write_fmt(format_args!(
            "[{:5}.{:06}] ",
            current_time / 1000000000,
            (current_time / 1000) % 1000000
        ));
        _ = writer.write_fmt(format_args!($prefix));
        _ = writer.write_fmt(format_args!($($arg)*));
        _ = writer.write_fmt(format_args!($suffix));
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print_inner!("", "", $($arg)*);
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::print_inner!("warn: ", "", $($arg)*);
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::print_inner!("error: ", "", $($arg)*);
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        print!("{:#?}\n", $($arg)*);
    });
}
