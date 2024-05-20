use serde;
use serde_json;
use std::fs;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub fields: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            fields: vec![
                "name".to_string(),
                "account_dbid".to_string(),
                "damage_dealt_pb".to_string(),
                "survived_pc".to_string(),
                "wr_pc".to_string(),
                "frags_pb".to_string(),
                "xp_pb".to_string(),
                "vehicle_shots_pb".to_string(),
                "vehicle_hits_pb".to_string(),
                "vehicle_explosion_hits_pb".to_string(),
                "vehicle_pierced_pb".to_string(),
                "sniper_damage_dealt_pb".to_string(),
                "vehicle_incoming_hits_pb".to_string(),
                "pierced_received_pb".to_string(),
                "explosion_hits_received_pb".to_string(),
                "no_damage_direct_hits_received_pb".to_string(),
                "potential_damage_received_pb".to_string(),
                "damage_blocked_by_armor_pb".to_string(),
                "spotted_pb".to_string(),
                "damaged_pb".to_string(),
                "killed_pb".to_string(),
                "damage_assisted_pb".to_string(),
                "damage_assisted_stun_pb".to_string(),
                "stun_num_pb".to_string(),
                "capture_points_pb".to_string(),
                "defence_points_pb".to_string(),
                "mileage_pb".to_string(),
                "accuracy_pc".to_string(),
                "penetration_pc".to_string(),
            ],
        }
    }

    pub fn save(&self) -> Result<(), serde_json::Error> {
        let buf = serde_json::to_string_pretty(self)?;
        fs::write("./config.json", &buf).unwrap();

        Ok(())
    }
}
