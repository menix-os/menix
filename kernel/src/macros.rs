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
macro_rules! current_module_name {
    () => {{
        let path = module_path!().as_bytes();
        let mut idx = path.len() - 1;
        let mut result = path;
        while idx > 0 {
            if path[idx] == b':' {
                result = path.split_at(idx + 1).1;
                break;
            }
            idx -= 1;
        }

        match str::from_utf8(result) {
            Ok(x) => x,
            Err(_) => panic!("Parsing error. FIXME"),
        }
    }};
}

#[macro_export]
macro_rules! log_inner {
    ($prefix:expr, $suffix:literal, $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
        let current_time = $crate::generic::clock::get_elapsed();
        _ = writer.write_fmt(format_args!(
            "[{:5}.{:06}] ",
            current_time / 1_000_000_000,
            (current_time / 1000) % 1_000_000
        ));
        const NAME: &str = current_module_name!();
        _ = writer.write_fmt(format_args!("{}: ", NAME));
        _ = writer.write_fmt(format_args!($prefix));
        _ = writer.write_fmt(format_args!($($arg)*));
        _ = writer.write_fmt(format_args!($suffix));
    });
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[0m", "\x1b[0m\n", $($arg)*);
    });
}

/// Logs a warning.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[1;33m", "\x1b[0m\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[1;31m", "\x1b[0m\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        $crate::log_inner!("", "", "{:#?}\n", $($arg)*);
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
/// The order in which hooked functions are called is not guaranteed.
///
/// # Safety
///
/// The caller must ensure that this call follows the following rules:
/// - The function may not allocate any heap memory.
/// - The function may not reference [`BootInfo`].
#[macro_export]
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
#[macro_export]
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
#[macro_export]
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

            $crate::init_call!(__init_call_wrapper);
        };
    };
}
