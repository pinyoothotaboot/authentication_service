use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Mobile {
    pub mobile_number : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Verify {
    pub mobile_number : String,
    pub access_token : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Info {
    pub id : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct InfoAuthorize {
    pub id : String,
    pub mobile : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Otp {
    pub otp_code : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Login {
    pub mobile_number : String,
    pub otp_code : String,
    pub role : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Event {
    pub otp_code : String,
    pub expired : u64,
    pub version : u64,
    pub event_type : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct State {
    pub role : String,
    pub access_token : String,
    pub refresh_token : String,
    pub created : u64,
    pub updated : u64
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Authentication {
    pub id : String,
    pub version : u64,
    pub mobile_number : String,
    pub events : Vec<Event>,
    pub state : State,
    pub deleted : bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub enum Data {
    Info,
    InfoAuthorize
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Payload {
    pub message : String,
    pub data : Vec<InfoAuthorize>
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Response {
    pub code : u64,
    pub success : bool,
    pub payload : Payload
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Claims {
    sub: String,
    role : String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

impl Claims {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `OffsetDateTime` contains a microsecond
    /// field but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes
    /// the timestamps.
    pub fn new(sub: String,role : String, iat: OffsetDateTime, exp: OffsetDateTime) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        Self { sub,role, iat, exp }
    }
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
