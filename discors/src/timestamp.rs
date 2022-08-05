use crate::snowflake::{Snowflakable, Snowflake};

/// A timestamp, representes a point in time, represented by a positive number of milliseconds since Janurary 1st 1970 UTC.
/// This is known as "unix milliseconds".
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Timestamp(u64); // an amount of unix miliseconds seconds

impl Timestamp {
    /// Construct a new timestamp from an amount of unix milliseconds.
    pub const fn new(s: u64) -> Self {
        Self(s)
    }
    /// Construct a new timestamp from an amount of unix seconds
    pub const fn from_unix_secs(s: u64) -> Self {
        Self(s * 1000)
    }
    /// Retrive a timestamp's unix milliseconds
    pub const fn to_unix_millis(&self) -> u64 {
        self.0
    }
    /// Retrive a timestamp's unix seconds
    pub const fn to_unix_seconds(&self) -> u64 {
        self.0 / 1000
    }
}

impl<T> From<Snowflake<T>> for Timestamp
where
    T: Snowflakable,
{
    fn from(s: Snowflake<T>) -> Self {
        s.to_time()
    }
}
