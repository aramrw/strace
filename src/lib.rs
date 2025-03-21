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
    ///
    /// // 2025-03-21 14:52:40.011389900 -07:00 DEBUG: serving...
    /// dbug!("serving...")
    /// ```
    #[macro_export]
    macro_rules! dbug {
        ($msg:expr) => {{
            let dt: $crate::chrono::DateTime<$crate::chrono::Local> = $crate::chrono::Local::now();
            $crate::color_print::cprintln!("{} <b>DEBUG</>: {}", dt, $msg);
        }};
        ($msg:expr, $title:expr) => {{
            let dt: $crate::chrono::DateTime<$crate::chrono::Local> = $crate::chrono::Local::now();
            $crate::color_print::cprintln!("{} <b>DEBUG</> <bold>{}</bold>: {}", dt, $title, $msg);
        }};
    }

    /// Error Date-Time
    ///
    /// # Example
    /// ```
    /// use strace::err;
    ///
    /// // 2025-03-21 14:52:40.011389900 -07:00 ERROR: serving...
    /// err!("fatal...")
    /// // output:
    /// ```
    #[macro_export]
    macro_rules! err {
        ($msg:expr) => {{
            let dt: $crate::chrono::DateTime<$crate::chrono::Local> = $crate::chrono::Local::now();
            $crate::color_print::cprintln!("{} <r>ERROR</>: {:#?}", dt, $msg);
        }};
        ($msg:expr, $title:expr) => {{
            let dt: $crate::chrono::DateTime<$crate::chrono::Local> = $crate::chrono::Local::now();
            $crate::color_print::cprintln!(
                "{} <r>ERROR</> <bold>{}</bold>: {:#?}",
                dt,
                $title,
                $msg
            );
        }};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tdbug() {
        dbug!("hello world");
    }
    #[test]
    fn terr() {
        err!("io error #53", "fatal");
    }
}
