const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() -> () {
    to_time(3600);
    ()
}

fn to_time(seconds: u32) -> String {
    let hours = seconds / ONE_HOUR_IN_SECONDS;
    let remaining_seconds = seconds % ONE_HOUR_IN_SECONDS;
    let minutes = remaining_seconds / 60;

    format!("{} hour(s) and {} minute(s)", hours, minutes)
}

#[cfg(test)]
mod tests {
    use super::to_time;

    #[test]
    fn basic() {
        assert_eq!(to_time(3_600), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_601), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_500), "0 hour(s) and 58 minute(s)");
        assert_eq!(to_time(323_500), "89 hour(s) and 51 minute(s)");
        assert_eq!(to_time(0), "0 hour(s) and 0 minute(s)");
    }
}
