// ess/src/varload.rs
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Canmi

use crate::config;
use rfs_utils::{log, LogLevel};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct CommonConfig {
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RfsdConfig {
    pub unix_socket: String,
}

// Make Config clonable so we can pass it around easily
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub common: CommonConfig,
    pub rfsd: RfsdConfig,
}

pub fn load_config(path_str: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new(path_str);

    if !path.exists() {
        log(
            LogLevel::Warn,
            &format!("Config file not found at {}. Creating default.", path_str),
        );
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let default_config_content = config::generate_default_config();
        fs::write(path, default_config_content)?;
        log(LogLevel::Info, "Default config file created.");
    }

    log(LogLevel::Info, &format!("Loading config from {}", path_str));
    let content = fs::read_to_string(path)?;
    let parsed_config: Config = toml::from_str(&content)?;

    log(LogLevel::Info, "Configuration loaded successfully.");

    // Return the parsed config directly
    Ok(parsed_config)
}