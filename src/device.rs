mod command;
mod status;

use command::Command;
use reqwest::Response;
use status::StateResponse;
pub use status::{State, Status};

use crate::error::Error;
use serde::Deserialize;
use std::any::Any;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};

pub struct BluOS {
    hostname: String,
    port: u16,

    client: reqwest::Client,
}

impl BluOS {
    pub async fn new() -> Result<BluOS, Error> {
        let (hostname, port) = BluOS::discover().await?;
        Ok(BluOS {
            hostname,
            port,
            client: reqwest::Client::new(),
        })
    }

    pub async fn new_from_ip(addr: Ipv4Addr) -> Result<BluOS, Error> {
        Ok(BluOS {
            hostname: addr.to_string(),
            port: 11000,
            client: reqwest::Client::new(),
        })
    }

    fn cmd(&self, action: &str) -> Command {
        Command::new(&self.hostname, self.port, action)
    }

    async fn discover() -> Result<(String, u16), Error> {
        let (tx, mut rx) = mpsc::channel(200);
        let (ctx, crx) = std::sync::mpsc::channel();

        tokio::task::spawn_blocking(move || {
            let mut browser = MdnsBrowser::new(ServiceType::new("musc", "tcp").unwrap());

            browser.set_service_discovered_callback(Box::new(
                move |result: zeroconf::Result<ServiceDiscovery>,
                      _context: Option<Arc<dyn Any>>| {
                    let res = result.unwrap();
                    let _ = tx.blocking_send(res);
                },
            ));

            let event_loop = browser.browse_services().unwrap();

            loop {
                event_loop.poll(Duration::from_millis(500)).unwrap();

                match crx.try_recv() {
                    Ok(_) => return,
                    Err(e) => match e {
                        std::sync::mpsc::TryRecvError::Empty => {}
                        std::sync::mpsc::TryRecvError::Disconnected => return,
                    },
                }
            }
        });

        let m = rx.recv().await.ok_or(Error::NoBluOSError)?;

        ctx.send(true)?;

        Ok((m.address().clone(), m.port().clone()))
    }

    pub async fn command(&self, cmd: Command) -> Result<Response, Error> {
        Ok(self.client.get(cmd.build()).send().await?)
    }

    pub async fn command_response<'a, T: Deserialize<'a>>(&self, cmd: Command) -> Result<T, Error> {
        let t = self.client.get(cmd.build()).send().await?.text().await?;
        dbg!(&t);
        Ok(serde_xml_rs::from_str(&t)?)
    }

    pub async fn status(&self) -> Result<Status, Error> {
        let status: Status = self.command_response(self.cmd("Status")).await?;

        Ok(status)
    }

    pub async fn update_library(&self) -> Result<(), Error> {
        self.command(self.cmd("Reindex")).await?;

        Ok(())
    }

    ///////////////////
    // Playback functions
    ///////////////////

    /// Plays whatever source is currently active
    pub async fn play(&self) -> Result<State, Error> {
        Ok(self.play_with_options(None, None, None).await?)
    }

    /// Play with the ability to define options
    /// - seek: time to seek in the track, max is total_length from status of the track
    /// - input_type:  Selects an input before starting playback.
    /// Possible values for inputType are: analog, spdif, hdmi or bluetooth.
    /// - index: For players with more than one input, this indicates which input of the specified
    /// type to play. Used only with inputType parameter. Default value is 1.
    pub async fn play_with_options(
        &self,
        seek: Option<i64>,
        input_type: Option<String>,
        index: Option<i64>,
    ) -> Result<State, Error> {
        let mut cmd = self.cmd("Play");

        cmd.add_optional("seek", seek);
        cmd.add_optional("inputType", input_type);
        cmd.add_optional("index", index);

        let state: StateResponse = self.command_response(cmd).await?;

        Ok(state.state)
    }
}
