#[macro_export]
macro_rules! static_assert {
    ($xs:expr) => {
        const _: () = assert!($xs, concat!("Assertion failed: \"", stringify!($xs), "\"!"));
    };
}

/// For regular per-CPU variables.
#[macro_export]
macro_rules! per_cpu {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => {
        #[unsafe(link_section = ".percpu")]
        $vis static $name: $crate::generic::percpu::PerCpuData<$t> =
            $crate::generic::percpu::PerCpuData::new($init);
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
macro_rules! print_old {
    ($($arg:tt)*) => ({
        $crate::print_inner!("", "", $($arg)*);
    });
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        $crate::print_inner!("", "\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::print_inner!("warn: ", "\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::print_inner!("error: ", "\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        $crate::print_inner!("", "", "{:#?}\n", $($arg)*);
    });
}

/// Asserts that a type implements a given trait.
#[macro_export]
macro_rules! assert_trait_impl {
    ($t:ty, $tr:tt) => {
        const _: () = {
            const fn assert_trait_impl<T: $tr>() {}
            assert_trait_impl::<$t>();
        };
    };
}

/// Hooks a function as an early init call.
///
/// # Safety
///
/// The caller must ensure that this call doesn't cause any heap memory allocations.
macro_rules! early_init_call {
    ($fun:ident) => {
        const _: () = {
            #[doc(hidden)]
            #[unsafe(link_section = ".early_array")]
            #[used]
            static __EARLY_INIT_CALL: unsafe fn() = $fun;
        };
    };
}

/// Hooks a function as an init call.
/// This gets called after all managers are available.
macro_rules! init_call {
    ($fun:ident) => {
        const _: () = {
            #[doc(hidden)]
            #[unsafe(link_section = ".init_array")]
            #[used]
            static __INIT_CALL: fn() = $fun;
        };
    };
}

/// Hooks a function as an init call if a command line option equals to true.
macro_rules! init_call_if_cmdline {
    ($opt:literal, $default:literal, $fun:ident) => {
        const _: () = {
            fn __init_call_wrapper() {
                use $crate::generic::boot::BootInfo;
                if BootInfo::get()
                    .command_line
                    .get_bool($opt)
                    .unwrap_or($default)
                {
                    $fun()
                }
            }

            #[doc(hidden)]
            #[unsafe(link_section = ".init_array")]
            #[used]
            static __INIT_CALL: fn() = __init_call_wrapper;
        };
    };
}
