use crate::error::Error;
use async_dnssd::StreamTimeoutExt;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct BluOS {
    hostname: String,
    port: u16,

    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BluOsStatus {
    pub status: Status,
}
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

impl BluOS {
    pub async fn new() -> Result<BluOS, Error> {
        let (hostname, port) = BluOS::discover().await?;
        dbg!(&hostname, &port);
        Ok(BluOS {
            hostname,
            port,
            client: reqwest::Client::new(),
        })
    }

    async fn discover() -> Result<(String, u16), Error> {
        let search_timeout = Duration::from_secs(10);
        let resolve_timeout = Duration::from_secs(3);

        let query = "_musc._tcp";

        //Need to pin this for some reason lol?
        let mut query_result = Box::pin(async_dnssd::browse(query).timeout(search_timeout));

        //Only one in my network lol this is so DIRTY but w/e
        let res = query_result.try_next().await?.ok_or(Error::NoBluOSError)?;

        let mut resolve = Box::pin(res.resolve().timeout(resolve_timeout));
        let bluos = resolve.try_next().await?.ok_or(Error::NoBluOSError)?;

        Ok((bluos.host_target, bluos.port))
    }

    pub async fn get_status(&self) -> Result<Status, Error> {
        let resp = self
            .client
            .get(format!("http://{}:{}/Reindex", self.hostname, self.port))
            .send()
            .await?;

        let text = &resp.text().await?;
        let status: Status = serde_xml_rs::from_str(text)?;

        Ok(status)
    }

    pub async fn update_library(&self) -> Result<(), Error> {
        self.client
            .get(format!("http://{}:{}/Reindex", self.hostname, self.port))
            .send()
            .await?;

        Ok(())
    }
}
