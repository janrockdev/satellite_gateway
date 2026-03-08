use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Mirrors the `co2data` table in the local MySQL / MariaDB database.
///
/// Column mapping (MySQL → Rust):
///   id          INT              → i32
///   sensorid    VARCHAR(30)      → String
///   gatewayid   VARCHAR(30)      → String
///   co2         INT              → i32
///   temperature FLOAT            → f32
///   humidity    FLOAT            → f32
///   voltage     SMALLINT UNSIGNED → u16
///   rssi        SMALLINT         → i16
///   posted      SMALLINT         → i16   (0 = pending, 1 = sent to cloud)
///   dateadded   DATETIME         → chrono::NaiveDateTime
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Co2Record {
    pub id: i32,
    pub sensorid: String,
    pub gatewayid: String,
    pub co2: i32,
    pub temperature: f32,
    pub humidity: f32,
    pub voltage: u16,
    pub rssi: i16,
    pub posted: i16,
    pub dateadded: NaiveDateTime,
}
