//! This is the main library for the `discors` crate.
#![warn(rust_2018_idioms, missing_debug_implementations)]
#![warn(clippy::unwrap_used, clippy::expect_used)]
// #![warn(missing_docs)]

#[cfg(feature = "v10")]
pub const API_VERSION: u8 = 10;
#[cfg(feature = "v10")]
pub const BASE_URL: &str = "https://discordapp.com/api/v10";

/// The main client used to connect to discord.
/// todo!
#[cfg(feature = "client")]
pub mod client;
/// A way to communicate presences to the discord api
#[cfg(feature = "presence")]
pub mod presence;

/// A general representation for colors in discord, represented using their hex value (24-bit rgb colors).
pub mod color;
pub use self::color as colour;
/// The Locale associated with a discord user, on the client-side this is the language and region
pub mod locale;
/// The models around discord data, like users, channels, guilds
pub mod models;
/// Discord Permissions representations, for channel, and server wide permissions.
pub mod permissions;
/// A snowflake, representing a discord ID.
pub mod snowflake;
/// A timestamp is a wrapper around a unix milliseconds time, that communicates with discord in various situations
pub mod timestamp;

/// The error type for errors from discord
pub mod error;

// aliases for being cool
/// A direct alias to [`self::snowflake::Snowflake`]
pub type Snowflake<T> = self::snowflake::Snowflake<T>;
/// A direct alias to [`self::color::Color`]
pub type Color = self::color::Color;
/// A direct alias to [`self::color::Color`]
pub type Colour = self::colour::Colour;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
pub mod utilities;
