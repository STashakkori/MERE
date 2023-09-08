use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::*;

pub struct Timestamp {
    seconds: i64,
    fractional: i64,
}

impl Timestamp {
    pub fn new(seconds: i64, fractional: i64) -> Self {
        Timestamp { seconds, fractional }
    }

    pub fn to_datetime(&self) -> Option<DateTime<Utc>> {
        let timestamp = self.seconds + (self.fractional / 1_000_000);
        let fractional_nanoseconds = (self.fractional % 1_000_000) * 1_000;
        Utc.timestamp_opt(timestamp, fractional_nanoseconds.try_into().unwrap()).single()
    }


    pub fn take() -> Self {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        Timestamp {
            seconds: since_epoch.as_secs() as i64,
            fractional: since_epoch.subsec_nanos() as i64
        }
    }

    pub fn get_seconds(&self) -> i64 {
        return self.seconds
    }

    pub fn get_fractional(&self) -> i64 {
        return self.fractional
    }
}
