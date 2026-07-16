use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use anyhow::{Context, anyhow};

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let host: IpAddr = env::var("HOST")
            .map_err(|_| anyhow!("`HOST` must be set"))?
            .parse()
            .context("Failed parsing HOST address")?;

        let port: u16 = env::var("PORT")
            .map_err(|_| anyhow!("`PORT` must be set"))?
            .parse()
            .map_err(|_| anyhow!("`PORT` must be a valid number"))?;

        Ok(Self {
            addr: SocketAddr::from((host, port)),
        })
    }
}
