use crate::{
    bitflags,
    color::Color,
    locale::Locale,
    optional_default,
    snowflake::{Snowflakable, Snowflake},
};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: Snowflake<User>,
    // rename username
    pub name: String,
    // Shown as username#discriminator in app.
    pub discriminator: u16,
    pub avatar: Option<String>, // avatar hash
    pub bot: bool,
    pub system: bool,
    pub mfa_enabled: bool,
    pub banner: Option<String>,
    pub accent_color: Option<Color>,
    pub locale: Locale,
    // rename verified
    pub email_verified: bool,
    pub email: Option<String>,
    pub flags: Userflags,
    // rename premium_type
    pub nitro_subscription: NitroSubscription,
}

bitflags!("The userflags for a [`User`] has. On the client-side, this refers to the badges.", "", "[Read more](https://discord.com/developers/docs/resources/user#user-object-user-flags)"; Userflags: u32: "crate::utilities::serde::deserialize_u32", "crate::utilities::serde::serialize_u32");

macro_rules! userflags {
    ($($flag:ident $($alias:ident)* = $val:expr $(; $doc:expr)?,)*) => {
        impl Userflags {
            /// Gives an empty set of userflags
            pub const EMPTY: Userflags = Userflags(0);
            $(
                #[doc = concat!(
                    optional_default!(concat!(
                        "The ",
                        stringify!($flag),
                        " userflag. ",
                    )$(, $doc)?),
                )]
                pub const $flag: Userflags = Userflags(1 << $val);
                $(
                    #[doc = concat!(
                        "An alias to the [`Userflags::",
                        stringify!($flag),
                        "`] user flag."
                    )]
                    pub const $alias: Userflags = Userflags::$flag;
                )*
            )*
        }
    };
}

userflags! {
    STAFF = 0,
    PARTNER = 1,
    HYPESQUAD = 2,
    BUG_HUNTER_LEVEL_1 = 3,
    HYPESQUAD_ONLINE_HOUSE_1 HYPESQUAD_BRAVERY = 6,
    HYPESQUAD_ONLINE_HOUSE_2 HYPESQUAD_BRILLIANCE = 7,
    HYPESQUAD_ONLINE_HOUSE_3 HYPESQUAD_BALANCE = 8,
    PREMIUM_EARLY_SUPPORTER EARLY_SUPPORTER = 9,
    TEAM_PSEUDO_USER = 10,
    BUG_HUNTER_LEVEL_2 = 14,
    VERIFIED_BOT = 16,
    VERIFIED_DEVELOPER = 17,
    CERTIFIED_MODERATOR = 18,
    BOT_HTTP_INTERACTIONS = 19,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NitroSubscription {
    None = 0,
    Classic = 1,
    Nitro = 2,
}

impl Snowflakable for User {
    fn id(&self) -> Snowflake<Self> {
        self.id.clone()
    }
}
