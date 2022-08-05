use crate::{bitflags, optional_default};

/// The size that is used to store a permission integer.
/// "Permissions are stored in a variable-length integer serialized into a string"
/// Currently the largest permission is 40 bit (fitting into a u64) but it might need exapnding.
/// "For long-term stability, it's recommended to deserialize the permissions using your preferred languages' Big Integer libraries"
/// Once that happens, this type can be changed internally.
/// Do not rely on this being a u64. You can always rely on it having a MAX const, debug, display, eq and ord and cmp implementations
pub type PermissionInteger = u64; // when changing this change line 14.

bitflags!("A wrapper around discord's permission system.",
    "",
    "[Read more](https://discord.com/developers/docs/topics/permissions)";
    Permissions: PermissionInteger: "crate::utilities::serde::deserialize_u64", "crate::utilities::serde::serialize_u64"
);

macro_rules! channeltype {
    (V) => {
        "Voice channels"
    };
    (T) => {
        "Text channels"
    };
    (S) => {
        "Stages"
    };
}

macro_rules! permissions {
    ($($perm:ident $($alias:ident)* = $val:expr; $doc:expr $(=> $($channeltype:ident),*)?;)*) => {
        impl Permissions {
            /// Gives an empty set of permissions
            pub const EMPTY: Self = Self::new(0);
            /// Gives a set of all permissions avalible
            pub const ALL: Self = Self::new(PermissionInteger::MAX);
            $(
                #[doc = concat!(
                    $doc,
                    ". ",
                    optional_default!(
                        "This permission works at a server level."
                        $(
                            ,
                            concat!(
                                "This permission is toggleable at a server level but can explicitly be set in ",
                                    $(
                                        channeltype!($channeltype),
                                        ", ",
                                    )+
                                "for specific usages in that channel."
                            )
                        )?
                    ),
                )]
                pub const $perm: Self = Self(1 << $val);
                $(
                    #[doc = concat!(
                        $doc,
                        ". This is an alias to [`Permissions::",
                        stringify!($perm),
                        "`]."
                    )]
                    pub const $alias: Self = Self::$perm;
                )*
            )*

            /// Constructs a new set of permission given the bits passed to it.
            /// [Read more](https://discord.com/developers/docs/topics/permissions)
            ///
            /// ```rust
            /// use discors::permissions::Permissions;
            ///
            /// let permissions = Permissions::new(0x802);
            ///
            /// assert_eq!(permissions, Permissions::SEND_MESSAGES | Permissions::KICK_MEMBERS);
            /// ```
            pub const fn new(bits: PermissionInteger) -> Self {
                Self(bits)
            }
        }
    };
}

// I include aliases here, because many people dont think of the permission as documented in the api.
// e.g. CREATE_INSTANT_INVITE, yes the invites are instant, but thats just creating invites, thus the aliases.
permissions! {
    CREATE_INSTANT_INVITE CREATE_INVITE INVITE = 0; "Allows creation of instant invites" => T, V, S;
    KICK_MEMBERS KICK = 1; "Allows kicking members";
    BAN_MEMBERS BAN = 2; "Allows banning members";
    ADMINISTRATOR ADMIN = 3; "Allows all permissions and bypasses channel permission overwrites";
    MANAGE_CHANNELS EDIT_CHANNELS = 4; "Allows management and editing of channels" => T, V, S;
    MANAGE_GUILD MANAGE_SERVER = 5; "Allows management and editing of the guild";
    ADD_REACTIONS REACT = 6; "Allows for the addition of reactions to messages" => T, V;
    VIEW_AUDIT_LOG AUDIT_LOG = 7; "Allows for viewing of audit logs";
    PRIORITY_SPEAKER = 8; "Allows for using priority speaker in a voice channel";
    STREAM VIDEO = 9; "Allows the user to go live, and use video" => V;
    VIEW_CHANNEL READ_MESSAGES = 10; "Allows guild members to view a channel, which includes reading messages in text channels and joining voice channels" => T, V, S;
    SEND_MESSAGES = 11; "Allows for sending messages in a channel and creating threads in a forum (does not allow sending messages in threads)" => V, T;
    SEND_TTS_MESSAGES = 12; "Allows for sending of `/tts` messages" => T, V;
    MANAGE_MESSAGES = 13; "Allows for deletion of other users messages and pinned messages" => T, V;
    EMBED_LINKS SHOW_GIFS = 14; "Links sent by users with this permission will be auto-embedded" => T, V;
    ATTACH_FILES FILES = 15; "Allows for uploading images and files" => T, V;
    READ_MESSAGE_HISTORY MESSAGE_HISTORY = 16; "Allows for reading of message history" => T, V;
    MENTION_EVERYONE MENTION_ALL_ROLES = 17; "Allows for using the `@everyone` tag to notify all users in a channel, and the `@here` tag to notify all online users in a channel. Also enables mentioning any role" => T, V, S;
    USE_EXTERNAL_EMOJIS EXTERNAL_EMOJIS = 18; "Allows the usage of custom emojis from other servers" => T, V;
    VIEW_GUILD_INSIGHTS VIEW_SERVER_INSIGHTS GUILD_INSIGHTS SERVER_INSIGHTS = 19; "Allows for viewing guild insights";
    CONNECT JOIN_VOICE = 20; "Allows for joining of a voice channel" => V, S; // I hope i never have to put a T here
    SPEAK = 21; "Allows for speaking in a voice channel" => V, S;
    MUTE_MEMBERS = 22; "Allows for muting members in a voice channel" => V, S;
    DEAFEN_MEMBERS = 23; "Allows for deafening of members in a voice channel" => V, S;
    MOVE_MEMBERS = 24; "Allows for moving of members between voice channels" => V, S;
    USE_VAD VOICE_ACTIVITY = 25; "Allows for using voice-activity-detection in a voice channel, if this is off push-to-talk is required" => V;
    CHANGE_NICKNAME = 26; "Allows for modification of own nickname";
    MANAGE_NICKNAMES = 27; "Allows for modification of other users nicknames";
    MANAGE_ROLES = 28; "Allows management and editing of roles" => T, V, S;
    MANAGE_WEBHOOKS = 29; "Allows management and editing of webhooks" => T, V;
    MANAGE_EMOJIS_AND_STICKERS MANAGE_EMOJIS = 30; "Allows management and editing of emojis and stickers";
    USE_APPLICATION_COMMANDS USE_COMMANDS = 31; "Allows members to use application commands, including slash commands and context menu commands." => T, V;
    REQUEST_TO_SPEAK = 32; "Allows for requesting to speak in stage channels. (This permission is under active development and may be changed or removed)" => S;
    MANAGE_EVENTS = 33; "Allows for creating, editing, and deleting scheduled events" => V, S;
    MANAGE_THREADS = 34; "Allows for deleting and archiving threads, and viewing all private threads" => T;
    CREATE_PUBLIC_THREADS = 35; "Allows for creating public and announcement threads" => T;
    CREATE_PRIVATE_THREADS = 36; "Allows for creating private threads" => T;
    USE_EXTERNAL_STICKERS EXTERNAL_STICKERS = 37; "Allows the usage of custom stickers from other servers" => T, V;
    SEND_MESSAGES_IN_THREADS = 38; "Allows for sending messages in threads" => T;
    USE_EMBEDDED_ACTIVITIES USE_ACTIVITIES ACTIVITIES = 39; "Allows for using Activities (applications with the `EMBEDDED` flag) in a voice channel" => V;
    MODERATE_MEMBERS TIMEOUT_MEMBERS = 40; "Allows for timing out users to prevent them from sending or reacting to messages in chat and threads, and from speaking in voice and stage channels";

}

impl Default for Permissions {
    /// Gives an empty set of permissions.
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::models::traits::BitFlag, PermissionInteger, Permissions};

    const ALL: Permissions = Permissions::ALL;
    const EMPTY: Permissions = Permissions::EMPTY;
    // example from api docs (Send Messages (0x800) and Add Reactions (0x40))
    const MY_PERMS: Permissions = Permissions::new(0x40 | 0x800);

    #[test]
    fn value() {
        assert_eq!(ALL.0, PermissionInteger::MAX);
        assert_eq!(EMPTY.0, 0);
        assert_eq!(MY_PERMS.0, 0x840);
    }

    #[test]
    fn includes() {
        // all has anything
        assert!(ALL.includes(Permissions::REACT));
        assert!(ALL.includes(Permissions::SEND_MESSAGES | Permissions::CREATE_PUBLIC_THREADS));

        // empty has nothing
        assert!(!EMPTY.includes(Permissions::BAN_MEMBERS));
        // except nothing
        assert!(EMPTY.includes(Permissions::EMPTY));

        // MY_PERMS has send messages
        assert!(MY_PERMS.includes(Permissions::SEND_MESSAGES));
        // and reactions
        assert!(MY_PERMS.includes(Permissions::ADD_REACTIONS));

        // but not admin
        assert!(!MY_PERMS.includes(Permissions::ADMINISTRATOR));
    }

    #[test]
    fn includes_any() {
        // all has anything
        assert!(ALL.includes_any(Permissions::REACT));
        assert!(ALL.includes_any(Permissions::SEND_MESSAGES | Permissions::CREATE_PUBLIC_THREADS));

        // empty has nothing, out of everything
        assert!(!EMPTY.includes_any(Permissions::ALL));
        // except nothing
        assert!(EMPTY.includes_any(Permissions::EMPTY));

        // MY_PERMS has send messages
        assert!(MY_PERMS.includes_any(Permissions::SEND_MESSAGES | Permissions::BAN_MEMBERS));
        // and reactions
        assert!(MY_PERMS.includes_any(Permissions::ADMINISTRATOR | Permissions::ADD_REACTIONS));
        // and any of all permissions
        assert!(MY_PERMS.includes_any(Permissions::ALL));

        // but not admin or change nickname
        assert!(!MY_PERMS.includes_any(Permissions::ADMINISTRATOR | Permissions::CHANGE_NICKNAME));
    }

    #[test]
    fn from() {
        assert_eq!(Permissions::new(0x804), 0x804.into());
        assert_eq!(Permissions::from(0x840), MY_PERMS);
        assert_eq!(PermissionInteger::from(MY_PERMS), 0x840);
    }

    #[test]
    fn deserialize() {
        let perms = serde_json::from_str::<Permissions>("2112").unwrap();
        let perms2 = serde_json::from_str::<Permissions>(&format!("\"{}\"", u64::MAX)).unwrap();
        assert_eq!(perms, MY_PERMS);
        assert_eq!(perms2, ALL);
    }

    #[test]
    fn serialize() {
        let perms = serde_json::to_string(&MY_PERMS).unwrap();
        assert_eq!(perms, "2112");
        let perms2 = serde_json::to_string(&ALL).unwrap();
        assert_eq!(perms2, format!("\"{}\"", u64::MAX));
    }
}
