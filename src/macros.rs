
#![macro_escape]

use std::fmt;

macro_rules! iocperrln (
    ($($arg:tt)*) => ( {
        format_args!(::macros::dumb_println, $($arg)*)
    } )
)
macro_rules! iocpabort (
    ($($arg:tt)*) => ( {
        ::macros::abort(format!($($arg)*).as_slice());
    } )
)
macro_rules! iocpassert (
    ( $arg:expr ) => ( {
        if !$arg {
            iocpabort!("assertion failed: {}", stringify!($arg));
        }
    } )
)

pub fn dumb_println(args: &fmt::Arguments) {
    use std::rt;
    let mut w = rt::Stderr;
    let _ = writeln!(&mut w, "{}", args);
}

pub fn abort(msg: &str) -> ! {
    let msg = if !msg.is_empty() { msg } else { "aborted" };
    let hash = msg.chars().fold(0, |accum, val| accum + (val as uint) );
    let quote = match hash % 10 {
        0 => "Winderp has derped.",
        1 => "I apologize for writing such terrible code.",
        2 => "Feel free to submit a PR to fix this.",
        3 => "How dare you encounter this error?",
        4 => "Once upon a time this program crashed. The end.",
        5 => "In the beginning there was the word, and that word was Rust.\
And then you came along and ruined everything.",
        6 => "This is not an easter egg.",
        7 => "Did the code cause the error, or did the error cause the code?",
        8 => "Please do not touch the red button.",
        9 => "You need more cowbell.",
        _ => "Congratulations, you failed at failing.",
    };
    iocperrln!("{}", "");
    iocperrln!("{}", quote);
    iocperrln!("{}", "");
    iocperrln!("fatal runtime error: {}", msg);

    abort();

    fn abort() -> ! {
        use std::intrinsics;
        unsafe { intrinsics::abort() }
    }
}
