use std::fmt;
use better_term::{Color, flush_styles};
use crate::DEBUG_OUTPUT;

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