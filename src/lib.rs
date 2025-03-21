use chrono::{DateTime, Local};
use color_print::cprintln;

/// Debug Date-Time
macro_rules! dbug {
    ($s:expr) => {{
        let dt: DateTime<Local> = Local::now();
        cprintln!("{} <b>DEBUG</>: {}", dt, $s);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tplt() {
        dbug!("hello world");
    }
}
