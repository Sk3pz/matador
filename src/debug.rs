use std::fmt;
use std::time::Duration;
use better_term::{Color, flush_styles};
use crate::DEBUG_OUTPUT;

pub fn timed<F: FnOnce() -> R, R>(f: F) -> (R, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

pub fn _debug_print(args: fmt::Arguments) {
    if !DEBUG_OUTPUT {
        return;
    }
    println!(
        "{}DBG > {}{}",
        Color::BrightBlack,
        Color::White,
        args,
    );
    flush_styles();
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        $crate::debug::_debug_print(format_args!($($arg)*));
    };
}