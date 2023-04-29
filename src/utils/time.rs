use std::time::SystemTime;
use chrono::{NaiveDateTime, Utc};

pub fn get_current_timestamp()->i64{
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64).unwrap()
}

pub fn get_current_timestamp_str()->NaiveDateTime{
    Utc::now().naive_utc()
}