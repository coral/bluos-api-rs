use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub etag: String,
    ////////////////
    // Volume
    /////////////////
    /// The player volume level in percentage
    /// -1 means player volume fixed.
    pub volume: i64,
    /// Volume in decibel
    #[serde(rename = "db")]
    pub volume_decibel: i64,
    /// Mute state. Set to 1 if volume is muted
    pub mute: u8,
    /// If the player is muted, then this contains the unmuted volume level.
    /// Values are from 0 to 100.
    #[serde(rename = "muteVolume")]
    pub muted_volume: i64,
    /// If the player is muted, then this contains the unmuted volume in dB.
    #[serde(rename = "muteDb")]
    pub muted_decibel: i64,

    ////////////////
    // Playback
    /////////////////
    /// The title of the current playing audio track. Also see title1 attribute.
    pub name: Option<String>,
    /// Album name of the current active track. Also see title1 attribute.
    pub album: Option<String>,
    /// Artist name of the current active track. Also see title1 attribute.
    pub artist: Option<String>,
    /// Total length of the current track, in seconds
    #[serde(rename = "totlen")]
    pub total_length: Option<i64>,
    /// The number of seconds the current audio track has been played
    #[serde(rename = "secs")]
    pub seconds_played: Option<i64>,

    /// 0, 1, or 2. 0 means repeat play queue, 1 means repeat a track, and 2 means repeat off
    pub repeat: u8,
    /// 0 or 1. 0 means shuffle off and 1 means shuffle on
    pub shuffle: u8,

    /// The position of the current track in the play queue. Also see streamUrl.
    #[serde(rename = "song")]
    pub song_queue_position: i64,
    /// Quality of the playing source audio:
    /// • cd - losless audio at CD quality
    /// • hd – lossless audio with higher resolution that CD quality or samplerate of 88200 samples/s or more
    /// • dolbyAudio – DolbyDigital or AC3
    /// • mqa – valid MQA audio decoded
    /// • mqaAuthored - valid MQA-Authored audio decoded
    /// A numeric value is the approximate bitrate of a compressed audio source quality of the file.
    pub quality: Option<String>,
    #[serde(rename = "fn")]
    pub filename: Option<String>,
    ////////////////
    // Display
    /////////////////
    /// URL of image associated with the current audio (album, station, input, etc.)
    pub image: Option<String>,
    /// The first line of information describing the current audio.
    /// title1, title2 and title3 MUST be used as the text of any UI that displays three lines of now-playing
    // metadata. Do not use values such as album, artist and name.
    pub title1: Option<String>,
    /// The second line of information describing the current audio.
    pub title2: Option<String>,
    /// The third line of information describing the current audio.
    pub title3: Option<String>,

    /// The first of two lines describing the current audio.
    ///  twoline_title1 & twoline_title2, if present, MUST be used as the text of any UI that displays two
    /// lines of now-playing metadata.
    pub twoline_title1: Option<String>,
    /// The second of two lines describing the current audio.
    pub twoline_title2: Option<String>,

    /// What the player displays currently?
    pub current_image: Option<String>,

    ////////////////
    // Group
    /////////////////
    /// Name of the group. The player must be the primary player in the group.
    pub group_name: Option<String>,
    /// Volume level of the group. The player must be the primary player in the group
    pub group_volume: Option<String>,

    ////////////////
    // Abiltes
    /////////////////
    pub actions: Option<Actions>,
    pub can_seek: u8,
    pub can_move_playback: bool,

    ////////////////
    // System
    /////////////////
    ///URL for a pop up notification
    notify_url: Option<String>,

    pub mode: i64,
    /// The unique play queue id. It matches the id attribute of the /Playlist response. If
    /// the play queue is changed this number will change
    pub pid: i64,
    /// The unique preset id. It matches the prid attribute in the /Presets response. If a
    /// preset is changed this number will change indicating that any cached response to
    /// /Presets should be purged.
    pub prid: u8,

    pub sid: i64,
    /// The minutes remaining before the sleep timer activates.
    pub sleep: Option<i64>,
    /// The current player state. It could be play, pause, stop, stream, connecting, etc.
    /// /Play can be used to resume when in a pause state but not when in stop state
    pub state: String,
    pub stream_url: Option<String>,
    pub sync_stat: i64,

    ////////////////
    // Undocumented
    /////////////////
    pub cursor: Option<i64>,
    /// Most likely inidicating if the player is currently indexing
    pub indexing: i64,
    pub mid: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    pub action: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub hide: u8,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Stop,
    Play,
    Pause,
    Stream,
    Connecting,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Quality {
    Cd,
    Hd,
    DolbyAudio,
    Mqa,
    MqaAuthored,
}
