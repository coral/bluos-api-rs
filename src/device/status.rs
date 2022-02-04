use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub etag: String,

    //Playback
    pub name: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    #[serde(rename = "totlen")]
    pub total_length: Option<i64>,

    pub quality: Option<Quality>,
    #[serde(rename = "fn")]
    pub filename: Option<String>,

    //Display
    pub image: String,
    pub title1: Option<String>,
    pub title2: Option<String>,
    pub title3: Option<String>,

    pub actions: Option<Actions>,
    pub can_seek: u8,
    pub current_image: Option<String>,
    pub cursor: i64,
    pub db: i64,
    pub indexing: i64,
    pub mid: i64,
    pub mode: i64,
    pub mute: u8,
    pub pid: i64,
    pub prid: u8,
    pub repeat: u8,
    pub shuffle: u8,
    pub sid: i64,
    pub sleep: bool,
    pub song: i64,
    pub state: State,
    pub stream_url: Option<String>,
    pub sync_stat: i64,
    pub volume: i64,
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
