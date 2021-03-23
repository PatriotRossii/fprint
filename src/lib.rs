//! This crate entroduces [`fprint!`] macro which provides flushed print

/// Prints to the standard ouput and flushes it.  
/// ## Usage
/// ```rust
/// use std::io::Read;
///
/// fn main() 
///     let mut buffer = String::new();
///     fprint!("Enter your number: ");
///     io::stdin().read_line(&mut buffer);
/// }
#[macro_export]
macro_rules! fprint {
    ($($arg:tt)*) => (print!($($arg)*); <std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout()).unwrap(); );
}
