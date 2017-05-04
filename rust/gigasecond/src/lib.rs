extern crate chrono;
use chrono::*;

pub fn after(datetime: chrono::DateTime<UTC>) -> chrono::DateTime<UTC> {
    return datetime + Duration::seconds(1_000_000_000);
}
