#![cfg(feature = "discover")]
use crate::error::Error;
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};

pub struct DiscoveredBluOSDevice {
    pub name: String,
    pub hostname: String,
    pub port: u16,
}

pub struct Discovery {
    cancel: Option<std::sync::mpsc::Sender<bool>>,
}

impl Discovery {
    pub fn new() -> Discovery {
        Discovery { cancel: None }
    }
    /// Discover uses mDNS to scan the network for BluOS devices
    /// Returns a Tokio channel that streams results as they are found
    ///
    /// The discovery process is cancelled on drop
    pub async fn discover(&mut self) -> Result<Receiver<DiscoveredBluOSDevice>, Error> {
        //Check if we're already doing this
        if self.cancel.is_some() {
            return Err(Error::AlreadyDiscovering);
        }

        let (tx, rx) = mpsc::channel(200);
        let (ctx, crx): (
            std::sync::mpsc::Sender<bool>,
            std::sync::mpsc::Receiver<bool>,
        ) = std::sync::mpsc::channel();
        self.cancel = Some(ctx);

        tokio::task::spawn_blocking(move || {
            let mut browser = MdnsBrowser::new(ServiceType::new("musc", "tcp").unwrap());

            browser.set_service_discovered_callback(Box::new(
                move |result: zeroconf::Result<ServiceDiscovery>,
                      _context: Option<Arc<dyn Any>>| {
                    let res = result.unwrap();
                    let _ = tx.blocking_send(DiscoveredBluOSDevice {
                        name: res.name().clone(),
                        hostname: res.address().clone(),
                        port: *res.port(),
                    });
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

        Ok(rx)
    }
    /// Discover one is a helper function that scans the network and returns the FIRST BluOS device it finds.
    /// This is useful if you only have one BluOS device.
    pub async fn discover_one() -> Result<DiscoveredBluOSDevice, Error> {
        let mut d = Discovery::new();
        let mut c = d.discover().await?;

        let m = c.recv().await.ok_or(Error::NoBluOSError)?;

        Ok(m)
    }
}

impl Drop for Discovery {
    fn drop(&mut self) {
        match &self.cancel {
            Some(c) => {
                let _ = c.send(true);
            }
            None => {}
        }
    }
}
