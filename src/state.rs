use anyhow::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Button {
    pub label: String,
    pub command: String,
}

pub struct State {
    pub address: SocketAddr,
    pub title: String,
    pub buttons: HashMap<Uuid, Button>,
}

impl State {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        #[derive(Deserialize)]
        pub struct Config {
            pub address: SocketAddr,
            pub title: String,
            pub buttons: Vec<Button>,
        }

        let buffer = std::fs::read(path)?;
        let config: Config = toml::from_str(std::str::from_utf8(buffer.as_ref())?)?;
        Ok(State {
            address: config.address,
            title: config.title,
            buttons: config
                .buttons
                .into_iter()
                .map(|button| (Uuid::new_v4(), button))
                .collect(),
        })
    }
}
