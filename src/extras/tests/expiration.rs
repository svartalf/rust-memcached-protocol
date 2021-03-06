use std::time::{SystemTime, Duration, UNIX_EPOCH};

use extras::Expiration;
use extras::traits::MAX_SECONDS;

#[test]
fn test_conversion_small() {
    assert_eq!(3600u32, u32::get_timeout(&3600));
}

#[test]
fn test_conversion_equal_to_limit() {
    let limit: u32 = MAX_SECONDS;
    assert_eq!(limit, u32::get_timeout(&limit));
}

#[test]
fn test_conversion_into_timestamp() {
    let value = MAX_SECONDS + 1;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expected = now.checked_add(Duration::new(value as u64, 0)).unwrap().as_secs() as u32;
    assert_eq!(expected, u32::get_timeout(&value));
}

#[test]
fn test_simple_timeout() {
    let timeout: u32 = 3600;

    assert_eq!(3600u32, timeout.get_timeout());
}

#[test]
fn test_more_that_30_days_timeout() {
    // Since this timeout is bigger than "30 days in seconds",
    // conversion should return current timestamp + 1024 seconds.
    let timeout: u32 = 60 * 60 * 24 * 30 + 1024;

    let now = SystemTime::now();
    let ts = now.duration_since(UNIX_EPOCH).unwrap()
        .checked_add(Duration::new(timeout as u64, 0)).unwrap().as_secs() as u32;
    let difference = ts - timeout.get_timeout();
    // Stupid way to check that conversion into u32 returned
    // value almost equal to expected timestamp
    assert!(difference == 0 || difference == 1);
}

#[test]
fn test_small_duration() {
    let timeout = Duration::new(3600, 0);

    assert_eq!(3600u32, timeout.get_timeout());
}
