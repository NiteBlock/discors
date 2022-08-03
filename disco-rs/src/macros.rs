// yes i know there is a bitflags crate but this is superior
#[macro_export]
#[doc(hidden)]
macro_rules! bitflags {
    // $f is the bitflags type (has a new method)
    // $i is the inner type (like u32)
    ($($doc:expr),*; $f:ident: $i:ident : $deserializer:literal, $serializer:literal) => {
        #[doc = concat!($(
            $doc,
            "\n",
        )*)]
        #[derive(::std::fmt::Debug, ::std::cmp::PartialEq, ::std::clone::Clone, Copy, ::std::cmp::Eq, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $f(#[serde(deserialize_with = $deserializer, serialize_with = $serializer)] pub $i);
        impl ::std::ops::BitOr for $f {
            type Output = $f;
            fn bitor(self, other: $f) -> Self::Output {
                Self(self.0 | other.0)
            }
        }
        impl ::std::ops::BitAnd for $f {
            type Output = $f;
            fn bitand(self, other: $f) -> Self::Output {
                Self(self.0 & other.0)
            }
        }
        impl ::std::ops::BitXor for $f {
            type Output = $f;
            fn bitxor(self, other: $f) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }
        impl ::std::ops::BitOrAssign for $f {
            fn bitor_assign(&mut self, other: $f) {
                self.0 |= other.0;
            }
        }
        impl ::std::ops::BitAndAssign for $f {
            fn bitand_assign(&mut self, other: $f) {
                self.0 &= other.0;
            }
        }
        impl ::std::ops::BitXorAssign for $f {
            fn bitxor_assign(&mut self, other: $f) {
                self.0 ^= other.0;
            }
        }
        impl ::std::convert::From<$i> for $f {
            fn from(i: $i) -> Self {
                Self(i)
            }
        }
        impl ::std::convert::From<$f> for $i {
            fn from(f: $f) -> Self {
                f.0
            }
        }
        impl ::std::ops::Not for $f {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        impl $crate::models::traits::BitFlag for $f {
            #[doc = concat!(
                "Checks if a set of ",
                stringify!($f),
                " includes another set of ",
                stringify!($f),
            )]
            fn includes(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
            #[doc = concat!(
                "Checks if a set of ",
                stringify!($f),
                " includes another set of ",
                stringify!($f),
            )]
            fn includes_any(self, other: Self) -> bool {
                (self.0 & other.0) != 0 || other.0 == 0
            }
        }
        /*
        impl<'de> ::serde::Deserialize<'de> for $f {
            fn deserialize<D>(d: D) -> Result<Self, <D as ::serde::Deserializer<'de>>::Error> where D: ::serde::Deserializer<'de> {
                if let Ok(i) = <$i>::deserialize(d) {
                    return Ok(Self(i))
                } else if let Ok(i) = String::deserialize(d) {
                    if let Ok(val) = i.parse(){
                        return Ok(Self(val))
                    }
                }
                Err(::serde::de::Error::custom("invalid integer provided"))

            }
        }
        impl ::serde::Serialize for $f {
            fn serialize<S>(&self, _serializer: S) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error> where S: ::serde::Serializer {
                todo!()
            }
        }
        */
    };
}

// These are just for internal use for some sus work arounds
// im not sure if theres a better way to do this that isnt this ugly so please lmk
#[macro_export]
#[doc(hidden)]
macro_rules! optional_default {
    // this is a sus work around
    ($default:expr, $x:expr) => {
        $x
    };
    ($default:expr, $x:block) => {
        $x
    };
    ($default:expr) => {
        $default
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! consume {
    ($e:expr, $into:expr) => {
        $into
    };
    ($e:expr, $into:block) => {
        $into
    };
}
