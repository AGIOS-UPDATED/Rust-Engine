use chrono::{Utc, DateTime};

pub fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_time(time: DateTime<Utc>, format: &str) -> String {
    time.format(format).to_string()
}
