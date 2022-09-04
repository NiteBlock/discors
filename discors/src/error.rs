use std::{
    error::Error as StdError,
    fmt::{self, Display, Error as FormatError},
    io::Error as IoError,
    num::ParseIntError,
};

macro_rules! error_impl {
    ($($name:ident($x:ty) $( Std $($std:expr)?;)? $( From $($from:expr)?;)?,)*) => {
        #[derive(Debug)]
        #[non_exhaustive]
        pub enum Error {
            $(

                $name($x),
            )*
        }

        impl StdError for Error {
            fn source(&self) -> Option<&(dyn StdError + 'static)> {
                match self {
                    $(
                        $(
                            Error::$name(err) => Some(err),
                            $(consume!($std))?
                        )?
                    )*
                    _ => None,
                }
            }
        }

        impl Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Error::$name(err) => write!(f, "{}", err),
                    )*
                }
            }
        }

        $(
            $(
                impl From<$x> for Error {
                    fn from(err: $x) -> Self {
                        Error::$name(err)
                    }
                }
                impl TryFrom<Error> for $x {
                    type Error = &'static str;
                    fn try_from(err: Error) -> Result<Self, Self::Error> {
                        match err {
                            Error::$name(err) => Ok(err),
                            _ => Err(concat!(stringify!($x), " could not be parsed from Error as a different type is stored in Error")),
                        }
                    }
                }
                $(
                    consume!($from)
                )?
            )?
        )*
    };
}

error_impl! {
    Format(FormatError) Std; From;,
    Io(IoError) Std; From;,
    ParseInt(ParseIntError) Std; From;,
    ConfigurationError(&'static str),
}
