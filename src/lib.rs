use serde_json;
use std::fs;

pub mod cl_args;
pub mod replay_to_json;

pub fn load_replay(path: &str) -> Result<serde_json::Value, String> {
    let buf = fs::read(path).unwrap();

    replay_to_json::replay_to_json(&buf)
}
