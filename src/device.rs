mod status;

pub use status::Status;

use crate::error::Error;
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

    pub async fn get_status(&self) -> Result<Status, Error> {
        let resp = self
            .client
            .get(format!("http://{}:{}/Status", self.hostname, self.port))
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
