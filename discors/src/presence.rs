#[derive(Debug)]
pub struct Presence {
    pub since: u64,
    pub status: Status,
    pub activities: Vec<Activity>,
}

#[derive(Debug)]
pub enum Status {
    // online
    Online,
    // dnd
    DoNotDisturb,
    // idle
    Idle,
    // offline
    Offline,
}

impl Status {
    pub const DND: Status = Status::DoNotDisturb;
    // invalid naming convention for code readablility. This is effectively an alias to offline
    // `assert_eq!(Status::Offline, Status::Invlisible)`
    #[allow(non_upper_case_globals)] // sorry.
    pub const Invlisible: Status = Status::Offline;
}

#[derive(Debug)]
pub struct Activity {
    pub name: String,
    // #[serde(rename = "type")]
    pub ty: ActivityType,
    // stream url
    pub url: Option<String>,
    pub details: Option<String>,
    pub created_at: u64,
    pub timestamps: ActivityTimestamps,
    // pub emoji: Option<Emoji>
    // party_id?, size = [current_size, max_size]?
    pub party: (Option<String>, Option<(u64, u64)>),
    pub assets: ActivityAssets,
    pub buttons: Vec<ActivityButton>,
    pub instance: bool,
    // todo!()
    // add emoji, flags
}

#[derive(Debug, Default)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Default)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug)]
pub struct ActivityButton {
    pub lable: String,
    pub url: Option<String>,
}

#[derive(Debug)]
pub enum ActivityType {
    // 0
    Playing, // Playing game
    // 1
    Streaming, // Streaming game, (on twitch)
    // 2
    Listening,
    // 3
    Watching,
    // 4
    Custom,
    // 5 (honestly didnt know this exsited till now)
    Competing,
}
