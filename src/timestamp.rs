use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current timestamp in seconds since UNIX epoch
pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Converts a UNIX timestamp (seconds) to a (year, month, day) tuple
pub fn unix_to_ymd(ts: u64) -> (i32, u32, u32) {
    let days = (ts / 86_400) as i64; // seconds -> days
    unix_days_to_ymd(days)
}

/// Converts days since UNIX epoch to (year, month, day)
fn unix_days_to_ymd(mut days: i64) -> (i32, u32, u32) {
    days += 719468; // shift epoch from 1970-01-01 to algorithm's start
    let era = days / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y as i32 + (m <= 2) as i32;

    (year as i32, m as u32, d as u32)
}

/// Returns a human-readable YYYY-MM-DD string for a timestamp
pub fn timestamp_to_string(ts: u64) -> String {
    let (year, month, day) = unix_to_ymd(ts);
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_and_format() {
        let ts = now_timestamp();
        let s = timestamp_to_string(ts);
        println!("Timestamp: {ts}, formatted: {s}");
        assert!(s.len() == 10); // "YYYY-MM-DD"
    }

    #[test]
    fn test_known_date() {
        // UNIX timestamp for 2025-12-02 00:00:00 UTC
        let ts = 1764902400;
        let s = timestamp_to_string(ts);
        assert_eq!(s, "2025-12-02");
    }
}

