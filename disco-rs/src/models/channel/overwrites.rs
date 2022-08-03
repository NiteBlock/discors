use crate::{models::user::User, permissions::Permissions, Snowflake};

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionOverwrites(pub Vec<PermissionOverwrite>);

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub id: PermissionOverwriteType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionOverwriteType {
    User(Snowflake<User>),
    // Role(Snowflake<Role>),
}

// so the idea
// .create_text_channel(channel_name, |opts| {
//     opts.overwrite(user_id, allow, deny)
//         .overwrite_everyone(allow, deny)
//         .overwrite(role_id, allow, deny)
// // alternately
//         .add_overwrite(snowflake, permission_overwrite)
// // to directly add it if you already have a PermissionOverwrite object, and a snowflake.
// }).await;

// basically want to avoid overcomplication and instanciation in weird ways, just pass allow first then deny.
// another thing i was thinking of
// opts.overwrites(|ow| {
//     ow.id(snowflake).allow(allow1).allow(allow2).deny(deny1)...
// })
// using the BitOrAssign.
