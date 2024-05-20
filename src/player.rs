#[derive(Debug)]
pub struct Player {
    pub vehicle_type: String,
    pub name: String,
    pub fake_name: String,
    pub wtr: u64,
    pub is_alive: bool,
    pub max_health: u64,
    pub clan_abbrev: String,
    pub is_team_killer: bool,
    pub team: String,
    pub avatar_session_id: String,
}
