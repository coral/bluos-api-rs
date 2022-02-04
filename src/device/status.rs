use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    etag: String,
    actions: Actions,
    can_seek: u8,
    current_image: String,
    cursor: i64,
    db: i64,
    image: String,
    indexing: i64,
    mid: i64,
    mode: i64,
    mute: u8,
    pid: i64,
    prid: u8,
    repeat: u8,
    shuffle: u8,
    sid: i64,
    sleep: bool,
    song: i64,
    state: String,
    stream_url: String,
    sync_stat: i64,
    title1: String,
    title2: String,
    volume: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    action: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    hide: u8,
    name: String,
}
