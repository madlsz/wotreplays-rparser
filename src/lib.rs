pub mod cl_args;
pub mod config;
pub mod player;
pub mod replay_to_json;

use crate::cl_args::Args;
use crate::config::Config;
use serde_json;
use std::fs;

pub fn load_replay(path: &str) -> Result<serde_json::Value, String> {
    let buf = fs::read(path).unwrap();

    replay_to_json::replay_to_json(&buf)
}

pub fn cli(args: &Args, config: &Config) {
    0;
}

pub fn gui(args: &Args, config: &Config) {
    0;
}
