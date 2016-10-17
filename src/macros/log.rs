

// #[cfg(debug_assertions)]
#[macro_use]
macro_rules! debug {
    ( $($args:tt)* ) => {
        { use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[debug][{}:{}] {}: {}\n"
              , file!(), line!()
              , module_path!()
              , format_args!($($args)*)); }
    }
}

#[macro_use]
macro_rules! info {
    ( dots: $dots:expr, $msg:expr, status: $status:expr ) => {
        use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {} {}"
              , module_path!()
              , $msg, $status);
        print!("{:<38}{:>40}", concat!($dots, $msg), $status );
    };
    ( dots: $dots:expr, $($args:tt)* ) => {
        { use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {}"
              , module_path!()
              , format_args!($($args)*));
        print!("{}{}", $dots, format_args!($($args)*)); }
    };
    ( $($args:tt)* ) => {
        { use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {}"
              , module_path!()
              , format_args!($($args)*));
        print!( $($args)* ); }
    };
}

#[macro_use]
macro_rules! infoln {
    ( dots: $dots:expr, $msg:expr, status: $status:expr ) => {
        use core::fmt::Write;
        // TODO: only do this if we're in debug mode?

        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {} {}\n"
              , module_path!()
              , $msg, $status);
        println!("{:<38}{:>40}", concat!($dots, $msg), $status );
    };
    ( dots: $dots:expr, $($args:tt)* ) => {
        { use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {}\n"
              , module_path!()
              , format_args!($($args)*));
        println!("{}{}", $dots, format_args!($($args)*)); }
    };
    ( $($args:tt)* ) => {
        { use core::fmt::Write;
        // TODO: only do this if we're in debug mode?
        write!( $crate::arch::drivers::serial::COM1.lock()
              , "[info] {}: {}\n"
              , module_path!()
              , format_args!($($args)*));
        println!( $($args)* ); }
    };
}
