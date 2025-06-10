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
macro_rules! container_of {
    ($st: ty, $field: expr, $value: expr) => {{
        // TODO: Should probably check if value and field have the same type.
        let _ptr = $value as *mut u8;
        if _ptr.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (_ptr.sub(offset_of!($st, $field))) as *mut $st }
        }
    }};
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
        {
            let mut writer = $crate::generic::log::GLOBAL_LOGGERS.lock();
            let current_time = $crate::generic::clock::get_elapsed();
            _ = writer.write_fmt(format_args!(
                "[{:5}.{:06}] ",
                current_time / 1_000_000_000,
                (current_time / 1000) % 1_000_000
            ));
            const NAME: &str = $crate::current_module_name!();
            _ = writer.write_fmt(format_args!("{}: ", NAME));
            _ = writer.write_fmt(format_args!($prefix));
            _ = writer.write_fmt(format_args!($($arg)*));
            _ = writer.write_fmt(format_args!($suffix));
        }
    });
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[0m", "\x1b[0m\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[1;33m", "\x1b[0m\n", $($arg)*);
    });
}

#[macro_export]
macro_rules! status {
    ($($arg:tt)*) => ({
        $crate::log_inner!("\x1b[1;32m", "\x1b[0m\n", $($arg)*);
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

/// Hooks a function or closure into the init system.
#[macro_export]
macro_rules! init_stage {
    ($(
        $(#[depends($($dep:path),* $(,)?)])?
        $(#[entails($($rdep:path),* $(,)?)])?
        $vis:vis $name:ident : $display_name:expr => $func:expr;
    )*) => {
        $(
            #[used]
            #[doc(hidden)]
            #[unsafe(link_section = ".init")]
            $vis static $name: $crate::generic::init::Node =
                $crate::generic::init::Node::new($display_name, $func);

            $($(
                const _: () = {
                    #[used]
                    #[doc(hidden)]
                    static __DEPENDS_EDGE: $crate::generic::init::Edge =
                        $crate::generic::init::Edge::new(&$dep, &$name);

                    #[used]
                    #[doc(hidden)]
                    #[unsafe(link_section = ".init.ctors")]
                    static __DEPENDS_EDGE_CTOR: fn() = || __DEPENDS_EDGE.register();
                };
            )*)?

            $($(
                const _: () = {
                    #[used]
                    #[doc(hidden)]
                    static __ENTAILS_EDGE: $crate::generic::init::Edge =
                        $crate::generic::init::Edge::new(&$name, &$rdep);

                    #[used]
                    #[doc(hidden)]
                    #[unsafe(link_section = ".init.ctors")]
                    static __ENTAILS_EDGE_CTOR: fn() = || __ENTAILS_EDGE.register();
                };
            )*)?

        )*
    }
}
