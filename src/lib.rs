//! Lightweight tracing library

#![deny(missing_docs)]
pub use chrono;
pub use color_print;

mod macros {
    /// Debug Date-Time
    ///
    /// # Example
    /// ```
    /// use strace::dbug;
    /// dbug!("serving...")
    /// // output:
    /// ```
    #[macro_export]
    macro_rules! dbug {
        ($s:expr) => {{
            let dt: $crate::chrono::DateTime<$crate::chrono::Local> = $crate::chrono::Local::now();
            $crate::color_print::cprintln!("{} <b>DEBUG</>: {}", dt, $s);
        }};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tplt() {
        dbug!("hello world");
    }
}
