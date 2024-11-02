use clap::Parser;
use std::{net::Ipv4Addr, path::PathBuf};

use serde::{Deserialize, Serialize};

/// General server configuration
#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct Config {
    /// Path to the server configuration file.
    #[serde(skip_serializing)]
    #[arg(short = 'C', long, default_value = "./config.toml")]
    pub config_path: PathBuf,

    /// Address of the interface where to bind the server.
    #[arg(short = 'i', long, default_value = "127.0.0.1")]
    pub server_ip: Ipv4Addr,

    /// Port for the server to listen on.
    #[arg(short = 'p', long, default_value_t = 25565)]
    pub server_port: u16,

    /// Message of the day visible in the server browser.
    #[arg(short = 'm', long, default_value = "A Minecraft Server")]
    pub motd: String,

    /// A number of ticks per second.
    #[arg(short = 't', long, default_value_t = 20)]
    pub tps: u32,

    /// Max number of players simultaneously connected to the server.
    #[arg(short = 'M', long, default_value_t = 20)]
    pub max_players: u8,

    /// World level type.
    #[arg(short = 'L', long, default_value = "FLAT")]
    pub level_type: String,

    /// Default game mode.
    #[arg(short = 'G', long, default_value_t = 1)]
    pub game_mode: i32,

    /// Whether the PvP is enabled on the server.
    #[arg(short = 'P', long, default_value_t = true)]
    pub enable_pvp: bool,

    /// World difficulty.
    #[arg(short = 'D', long, default_value_t = 0)]
    pub difficulty: i8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path: "./config.toml".into(),
            server_ip: Ipv4Addr::LOCALHOST,
            server_port: 25565,
            motd: "A Minecraft Server".to_string(),
            tps: 20,
            max_players: 20,
            level_type: "FLAT".to_string(),
            game_mode: 1,
            enable_pvp: true,
            difficulty: 0,
        }
    }
}
