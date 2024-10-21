use std::ops::{Add, Div, Mul, Range, Sub};

pub mod app;
pub mod graphics;

pub fn map<T>(x: T, input_range: Range<T>, output_range: Range<T>) -> T
where
    T: Copy
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>,
{
    let (u, v) = (input_range.start, input_range.end);
    let (s, t) = (output_range.start, output_range.end);
    let m = (s - t) / (u - v);
    let b = (t * u - v * s) / (u - v);
    x * m + b
}

/// Add context to an anyhow error which includes a formatted error message, the
/// file path, and the line number.
///
/// # Usage
///
/// Given a method which returns an anyhow result:
///
/// my_method()
///     .with_context(!trace("some error message {}", some_variable))?;
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        || {
            let res = format!("{}:{} - {}", file!(), line!(), format!($($arg)*));
            res
        }
    }}
}
