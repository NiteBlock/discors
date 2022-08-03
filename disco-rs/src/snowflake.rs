use std::{cmp::Ordering, fmt::Display, marker::PhantomData, str::FromStr};

use crate::timestamp::Timestamp;

/// A discord snowflake, represents any ID on discord.
/// These IDs are guaranteed to be unique across all of Discord, except in some unique scenarios in which child objects share their parent's ID.
/// Intern they are stored in 64 bit integers, [`disco_rs`] will do all the seralization automatically to convert it from and to strings.
///
#[derive(Debug, Clone, Copy)]
pub struct Snowflake<S>(u64, PhantomData<S>)
where
    S: Snowflakable;

impl<T> From<u64> for Snowflake<T>
where
    T: Snowflakable,
{
    fn from(s: u64) -> Self {
        Snowflake::new(s)
    }
}

impl<T> From<Snowflake<T>> for u64
where
    T: Snowflakable,
{
    fn from(s: Snowflake<T>) -> Self {
        s.0
    }
}

impl<T> FromStr for Snowflake<T>
where
    T: Snowflakable,
{
    type Err = <u64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snowflake::new(s.parse()?))
    }
}

impl<T> Display for Snowflake<T>
where
    T: Snowflakable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            return self.0.fmt(f);
        }
        let mut res = format!("{}", self.0);
        while res.len() < 19 {
            res = "0".to_string() + &res;
        }
        f.write_str(&res)
    }
}

impl<T> Snowflake<T>
where
    T: Snowflakable,
{
    /// Construct a new snowflake struct, given the snowflake itself
    pub const fn new(s: u64) -> Self {
        Self(s, PhantomData)
    }
    /// Convert the snowflake into a [`Timestamp`]
    pub const fn to_time(&self) -> Timestamp {
        // self.into() would not const, so we
        // this could really be un const but we are cooler than that
        // https://discord.com/developers/docs/reference#snowflakes-snowflake-id-format-structure-left-to-right
        Timestamp::new((self.0 >> 22) + 1420070400000)
    }
    /// Converts the given snowflake into a new type. The generic type parameter `U` specifies the snowflakable it should be of afterwards.
    pub fn convert<U>(&self) -> Snowflake<U>
    where
        U: Snowflakable,
    {
        Snowflake::<U>::new(self.0)
    }
}

impl<T> PartialEq for Snowflake<T>
where
    T: Snowflakable,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialOrd for Snowflake<T>
where
    T: Snowflakable,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        if self > rhs {
            Some(Ordering::Greater)
        } else if self == rhs {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

/// Something that is tied to a specific snowflake
pub trait Snowflakable {
    fn id(&self) -> Snowflake<Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{models::user::User, timestamp::Timestamp};

    use super::Snowflake;

    // whats great about using consts is that im pretty sure the compiler optimizes away the entirty of the tests, so they either pass or fail already before running.

    const MY_SNOWFLAKE: Snowflake<User> = Snowflake::new(445556389532925952);
    const ANOTHER_SNOWFLAKE: Snowflake<User> = Snowflake::new(0);
    // LAST_SNOWFLAKE will be the theoretical last snowflake supported by this libary (and discord as far as their docs say)
    // this tragic event will occur in may 15th 2154  12:35:11 am and 103 milli seconds
    const LAST_SNOWFLAKE: Snowflake<User> = Snowflake::new(u64::MAX);

    #[test]
    fn value() {
        assert_eq!(MY_SNOWFLAKE.0, 445556389532925952);
        assert_eq!(ANOTHER_SNOWFLAKE.0, 0);
        assert_eq!(LAST_SNOWFLAKE.0, u64::MAX);
    }

    #[test]
    fn timestamp() {
        assert_eq!(Timestamp::new(1526299321302), MY_SNOWFLAKE.into());
        assert_eq!(Timestamp::new(1420070400000), ANOTHER_SNOWFLAKE.into());
        assert_eq!(Timestamp::new(5818116911103), LAST_SNOWFLAKE.to_time());
    }

    #[test]
    fn from_str() {
        assert_eq!(
            MY_SNOWFLAKE,
            Snowflake::<User>::from_str("445556389532925952").unwrap()
        );
        // allow prepended 0's
        assert_eq!(
            ANOTHER_SNOWFLAKE,
            Snowflake::from_str("0000000000000000000").unwrap()
        );
        assert_eq!(
            LAST_SNOWFLAKE,
            Snowflake::from_str(&u64::MAX.to_string()).unwrap()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(MY_SNOWFLAKE.to_string(), "445556389532925952");
        assert_eq!(format!("{}", LAST_SNOWFLAKE), u64::MAX.to_string());
        assert_eq!(format!("{:#}", ANOTHER_SNOWFLAKE), "0000000000000000000");
    }
}
