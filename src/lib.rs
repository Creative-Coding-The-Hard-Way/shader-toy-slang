pub mod app;
pub mod graphics;

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
