use crate::{bitflags, consume, optional_default};

bitflags!("The Gateway Intents that are specified for a connection to discord", "", "[Read More](https://discord.com/developers/docs/topics/gateway#gateway-intents)"; Intents: u64: "crate::utilities::serde::deserialize_u64", "crate::utilities::serde::serialize_u64");

macro_rules! intents {
    ($($intent:ident = $val:expr $(; $doc:expr)? $(=> $($event:ident)*)? $(; $privileged:ident)?,)*) => {
        impl Intents {
            /// Gives all intents.
            pub const ALL: Self = Self(u64::MAX);

            /// Gives an empty set of intents
            pub const EMPTY: Self = Self(0);
            $(
                #[doc = concat!(
                    optional_default!(concat!(
                        "The ",
                        stringify!($intent),
                        " intent. ",
                        $(
                            consume!($privileged, "This is a [privileged intent](https://discord.com/developers/docs/topics/gateway#privileged-intents), and it is required to be explicitely enabled in the [discord developer portal](https://discord.com/developers/applications).\n"),
                        )?
                    )$(, $doc)?),
                    $(
                        "\nThis intent enables the folling events: ",
                        $("\n", stringify!($event)),*
                    )?
                )]
                pub const $intent: Intents = Intents(1 << $val);
            )*
            /// Gives the intents that are defined as privileged.
            pub const fn privileged() -> Self {
                Self($(
                    $(
                        (1 << $val) | consume!($privileged, 0) |
                    )?
                )* 0)
            }

            /// Gives the intents that are not defeined as privileged.
            pub const fn non_privileged() -> Self {
                Self(!Self::privileged().0)
            }

            /// Creates a new set of intents with the bits specified.
            pub const fn new(x: u64) -> Self {
                Self(x)
            }
        }
    };
}

intents! {
    GUILDS = 0 => GUILD_CREATE GUILD_DELETE GUILD_ROLE_CREATE GUILD_ROLE_UPDATE GUILD_ROLE_DELETE,
    GUILD_MEMBERS = 1 => GUILD_MEMBER_ADD GUILD_MEMBER_UPDATE GUILD_MEMBER_REMOVE THREAD_MEMBERS_UPDATE; privileged,
    GUILD_BANS = 2,
    GUILD_EMOJIS_AND_STICKERS = 3,
    GUILD_INTEGRATIONS = 4,
    GUILD_WEBHOOKS = 5,
    GUILD_INVITES = 6,
    GUILD_VOICE_STATES = 7,
    GUILD_PRESENCES = 8 => ; privileged,
    GUILD_MESSAGES = 9,
    GUILD_MESSAGE_REACTIONS = 10,
    GUILD_MESSAGE_TYPING = 11,
    DIRECT_MESSAGES = 12,
    DIRECT_MESSAGE_REACTIONS = 13,
    DIRECT_MESSAGE_TYPING = 14,
    MESSAGE_CONTENT = 15,
    GUILD_SCHEDULED_EVENTS = 16,
    AUTO_MODERATION_CONFIGURATION = 20,
    AUTO_MODERATION_EXECUTION = 21,
}

impl Default for Intents {
    /// Gives the default, non-privileged intents.
    /// If you are looking for empty intents, use [`Intents::EMPTY`]
    fn default() -> Self {
        Intents::non_privileged()
    }
}

#[cfg(test)]
mod test {
    use super::Intents;

    #[test]
    fn value() {
        assert_eq!(Intents::EMPTY.0, 0);
        assert_eq!(Intents::GUILD_EMOJIS_AND_STICKERS.0, 1 << 3);
    }

    #[test]
    fn bitor() {
        assert_eq!(
            Intents::GUILD_BANS | Intents::GUILD_MEMBERS,
            Intents::GUILD_MEMBERS | Intents::GUILD_BANS
        );

        assert_eq!(
            (Intents::DIRECT_MESSAGE_REACTIONS
                | Intents::GUILD_SCHEDULED_EVENTS
                | Intents::GUILD_BANS)
                .0,
            0b10010000000000100 // 1 << 13 | 1 << 16 | 1 << 2
        )
    }

    #[test]
    fn special() {
        assert_eq!(Intents::privileged().0, 1 << 1 | 1 << 8);
        assert_eq!(Intents::non_privileged().0, !(u64::MAX & (1 << 1 | 1 << 8))); // !(a intersect b) == (a bitxor b) (in this case)
        assert_eq!(
            Intents::privileged() | Intents::non_privileged(),
            Intents::ALL
        );
    }
}
