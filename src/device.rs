mod command;
mod responses;

use crate::error::Error;
use command::Command;
use reqwest::Response;
use responses::StateResponse;
pub use responses::{Browse, IdResponse, Playlist, PlaylistEntry, State, Status};
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr};

#[cfg(feature = "discover")]
use crate::DiscoveredBluOSDevice;

// Documented here
// https://bluos.net/wp-content/uploads/2021/03/Custom-Integration-API-v1.0_March-2021.pdf
#[derive(Debug)]
pub struct BluOS {
    hostname: String,
    port: u16,

    client: reqwest::Client,
}

impl BluOS {
    /// Create a new BluOS device from an Ipv4Addr
    ///
    /// - If you for some reason managed to make your BluOS device listen on another port, define it using custom_port
    pub fn new(addr: Ipv4Addr, custom_port: Option<u16>) -> Result<BluOS, Error> {
        let port = match custom_port {
            Some(v) => v,
            None => 11000,
        };

        Ok(BluOS {
            hostname: addr.to_string(),
            port,
            client: reqwest::Client::new(),
        })
    }

    pub fn with_socket_addr(addr: SocketAddr) -> Result<BluOS, Error> {
        Ok(BluOS {
            hostname: addr.ip().to_string(),
            port: addr.port(),
            client: reqwest::Client::new(),
        })
    }

    /// Create a new BluOS device from a discovered device
    #[cfg(feature = "discover")]
    pub fn new_from_discovered(d: DiscoveredBluOSDevice) -> Result<BluOS, Error> {
        Ok(BluOS {
            hostname: d.hostname,
            port: d.port,
            client: reqwest::Client::new(),
        })
    }

    fn cmd(&self, action: &str) -> Command {
        Command::new(&self.hostname, self.port, action)
    }

    /// Send your own command to the BluOS Device
    async fn command(&self, cmd: Command) -> Result<Response, Error> {
        Ok(self.client.get(cmd.build()).send().await?)
    }

    /// Send your own command to the BluOS device and expect a response
    /// The function is generic and uses the type to determine what struct to deserialize to
    async fn command_response<'a, T: Deserialize<'a>>(&self, cmd: Command) -> Result<T, Error> {
        let t = self.client.get(cmd.build()).send().await?.text().await?;
        Ok(serde_xml_rs::from_str(&t)?)
    }

    /// Get the current status of the BluOS device
    pub async fn status(&self) -> Result<Status, Error> {
        let status: Status = self.command_response(self.cmd("Status")).await?;

        Ok(status)
    }

    pub async fn browse(&self, key: Option<&str>) -> Result<Browse, Error> {
        let mut cmd = self.cmd("Browse");
        cmd.add_optional("key", key);
        let browse: Browse = self.command_response(cmd).await?;
        Ok(browse)
    }

    /// Re-indexes the library
    ///
    /// This function is why I wrote this wrapper and once that worked I figured
    /// why not just keep going and write the rest LOL
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
    /// Pause playback
    /// - toggle: If set to 1, then the current pause state is toggled.
    pub async fn pause(&self, toggle: bool) -> Result<State, Error> {
        let mut cmd = self.cmd("Pause");
        if toggle {
            cmd.add_param("toggle", 1);
        }
        let state: StateResponse = self.command_response(cmd).await?;
        Ok(state.state)
    }
    /// Stop playback
    pub async fn stop(&self) -> Result<State, Error> {
        let state: StateResponse = self.command_response(self.cmd("Stop")).await?;
        Ok(state.state)
    }
    /// Skip: Skip to the next audio track in the play queue
    pub async fn skip(&self) -> Result<IdResponse, Error> {
        let id: IdResponse = self.command_response(self.cmd("Skip")).await?;
        Ok(id)
    }
    /// Back: If a track is playing and has been playing for more than four seconds, then back, will return to the start of the track.
    /// Otherwise, the back command will go to the previous song in the current playlist. If on the first song in the playlist
    /// calling back will go to the last song. It will go to the previous or first track in the queue regardless of the state of
    /// the repeat setting.
    pub async fn back(&self) -> Result<IdResponse, Error> {
        let id: IdResponse = self.command_response(self.cmd("Back")).await?;
        Ok(id)
    }

    /// Shuffle
    /// The shuffle command creates a new queue by shuffling the current queue.
    /// The original (not shuffled) queue is retained for restore when shuffle is disabled.
    pub async fn shuffle(&self, enable: bool) -> Result<(), Error> {
        let mut cmd = self.cmd("Shuffle");
        if enable {
            cmd.add_param("state", 1 as u8);
        } else {
            cmd.add_param("state", 0 as u8);
        }
        self.command(cmd).await?;
        Ok(())
    }

    /// Repeat... repeats
    ///
    /// Takes RepeatSetting enum which defines what kind of repeat you need
    pub async fn repeat(&self, setting: RepeatSetting) -> Result<(), Error> {
        let mut cmd = self.cmd("Repeat");

        cmd.add_param("state", setting as u8);

        self.command(cmd).await?;
        Ok(())
    }

    ///////////////////
    // Play Queue Management
    ///////////////////

    /// Get the current play queue from the BluOS device
    pub async fn queue(&self, pagination: Option<Pagination>) -> Result<Playlist, Error> {
        let mut cmd = self.cmd("Playlist");
        match pagination {
            Some(p) => {
                cmd.add_param("start", p.start);
                cmd.add_param("end", p.end);
            }
            None => {}
        };
        let pl: Playlist = self.command_response(cmd).await?;

        Ok(pl)
    }

    /// Delete a song at POSITION
    pub async fn queue_delete_song(&self, position: u64) -> Result<(), Error> {
        let mut cmd = self.cmd("Delete");

        cmd.add_param("id", position);

        self.command(cmd).await?;
        Ok(())
    }

    /// Clear the play queue
    pub async fn queue_clear(&self) -> Result<(), Error> {
        let cmd = self.cmd("Clear");

        self.command(cmd).await?;
        Ok(())
    }
}

pub struct Pagination {
    start: u64,
    end: u64,
}

pub enum RepeatSetting {
    EntireQueue = 0,
    CurrentTrack = 1,
    Disable = 2,
}
