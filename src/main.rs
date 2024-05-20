use clap::Parser;

use wotreplays_rparser::cl_args::Args;
use wotreplays_rparser::load_replay;
use wotreplays_rparser::player::Player;

fn main() {
    let args = Args::parse();

    let path = "20240519_2045_poland-Pl21_CS_63_05_prohorovka.wotreplay";
    let replay = load_replay(&path).unwrap();

    let author_team = &replay[0]["personal"]["avatar"]["team"];
    let battle_winner = &replay[0]["common"]["winnerTeam"];

    let mut players: Vec<Player> = Vec::new();
    for (rep_id, _) in replay[1].as_object().unwrap().iter() {
        let vehicle_type = &replay[1][rep_id]["vehicleType"];
        if ["Observer", "Spectator"]
            .contains(&vehicle_type.as_str().unwrap().split(':').nth(1).unwrap())
        {
            continue;
        }
        let name = replay[1][rep_id]["name"].as_str().unwrap();
        let fake_name = replay[1][rep_id]["fakeName"].as_str().unwrap();
        let wtr = replay[1][rep_id]["wtr"].as_u64().unwrap_or(0u64);
        let is_alive = match replay[1][rep_id]["isAlive"].as_u64().unwrap() {
            0 => false,
            _ => true,
        };
        let max_health = replay[1][rep_id]["maxHealth"].as_u64().unwrap();
        let clan_abbrev = replay[1][rep_id]["clanAbbrev"].as_str().unwrap();
        let is_team_killer = match &replay[1][rep_id]["isTeamKiller"].as_u64().unwrap() {
            0 => false,
            _ => true,
        };
        let team = if &replay[1][rep_id]["team"] == battle_winner {
            "green"
        } else {
            "red"
        };
        let avatar_session_id = replay[1][rep_id]["avatarSessionID"].as_str().unwrap();

        players.push(Player {
            vehicle_type: vehicle_type.as_str().unwrap().to_string(),
            name: name.to_string(),
            fake_name: fake_name.to_string(),
            wtr,
            is_alive,
            max_health,
            clan_abbrev: clan_abbrev.to_string(),
            is_team_killer,
            team: team.to_string(),
            avatar_session_id: avatar_session_id.to_string(),
        });

        let spotted = &replay[0]["vehicles"][avatar_session_id][0]["spotted"];
    }

    println!("{players:?}");
    // for byte in &buf {
    //     print!("{:02x} ", byte);
    // }
}
