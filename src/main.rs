use clap::Parser;

use wotreplays_rparser::cl_args::Args;
use wotreplays_rparser::load_replay;
use wotreplays_rparser::player::Player;

fn get_players(replay: &serde_json::Value, buf: &mut Vec<Player>) {
    let author_team = replay[0]["personal"]["avatar"]["team"].as_u64().unwrap();
    let battle_winner = replay[0]["common"]["winnerTeam"].as_u64().unwrap();

    for (rep_id, _) in replay[1].as_object().unwrap().iter() {
        let path = &replay[1][rep_id];

        let vehicle_type = &path["vehicleType"];
        if ["Observer", "Spectator"]
            .contains(&vehicle_type.as_str().unwrap().split(':').nth(1).unwrap())
        {
            continue;
        }
        let name = path["name"].as_str().unwrap();
        let fake_name = path["fakeName"].as_str().unwrap();
        let wtr = path["wtr"].as_u64().unwrap_or(0u64);
        let is_alive = match replay[1][rep_id]["isAlive"].as_u64().unwrap() {
            0 => false,
            _ => true,
        };
        // let max_health = path["maxHealth"].as_u64().unwrap();
        let clan_abbrev = path["clanAbbrev"].as_str().unwrap();
        // let is_team_killer = match &path["isTeamKiller"].as_u64().unwrap() {
        //     0 => false,
        //     _ => true,
        // };
        let team = if &path["team"] == battle_winner {
            "green"
        } else {
            "red"
        };
        let avatar_session_id = path["avatarSessionID"].as_str().unwrap();

        let path = &replay[0]["vehicles"][avatar_session_id][0];

        let account_dbid = path["accountDBID"].as_u64().unwrap();
        let spotted = path["spotted"].as_u64().unwrap();
        let vehicle_num_captured = path["vehicleNumCaptured"].as_u64().unwrap();
        let damage_assisted_track = path["damageAssistedTrack"].as_u64().unwrap();
        let xp_penalty = path["xpPenalty"].as_u64().unwrap();
        let direct_team_hits = path["directTeamHits"].as_u64().unwrap();
        let damage_received = path["damageReceived"].as_u64().unwrap();
        let life_time = path["lifeTime"].as_u64().unwrap();
        let piercings_received = path["piercingsReceived"].as_u64().unwrap();
        let sniper_damage_dealt = path["sniperDamageDealt"].as_u64().unwrap();
        // let destructible_deaths = path["destructibleDeaths"].as_array().unwrap();
        let piercing_enemy_hits = path["piercingEnemyHits"].as_u64().unwrap();
        let damage_assisted_radio = path["damageAssistedRadio"].as_u64().unwrap();
        let mileage = path["mileage"].as_u64().unwrap();
        let stun_duration = path["stunDuration"].as_f64().unwrap();
        let piercings = path["piercings"].as_u64().unwrap();
        let damage_blocked_by_armor = path["damageBlockedByArmor"].as_u64().unwrap();
        let xp = path["xp"].as_u64().unwrap();
        let dropped_capture_points = path["droppedCapturePoints"].as_u64().unwrap();
        // let killer_id = path["killerID"].as_u64().unwrap();
        let xp_other = path["xp/other"].as_u64().unwrap();
        // let index = path["index"].as_u64().unwrap();
        let direct_hits_received = path["directHitsReceived"].as_u64().unwrap();
        let damage_received_from_invisibles = replay[0]["vehicles"][avatar_session_id][0]
            ["damageReceivedFromInvisibles"]
            .as_u64()
            .unwrap();
        let explosion_hits_received = path["explosionHitsReceived"].as_u64().unwrap();
        let achievement_xp = path["achievementXP"].as_u64().unwrap();
        // let death_reason = path["deathReason"].as_i64().unwrap();
        let capture_points = path["capturePoints"].as_u64().unwrap();
        let num_recovered = path["numRecovered"].as_u64().unwrap();
        let direct_enemy_hits = path["directEnemyHits"].as_u64().unwrap();
        // let max_health = path["maxHealth"].as_u64().unwrap();
        // let damage_event_list = path["damageEventList"].as_u64().unwrap();
        // let health = path["health"].as_u64().unwrap();
        // let stop_respawn = path["stopRespawn"].as_bool().unwrap();
        let achievement_credits = path["achievementCredits"].as_u64().unwrap();
        // let achievements = path["achievements"].as_array().unwrap();
        let xp_assist = path["xp/assist"].as_u64().unwrap();
        let shots = path["shots"].as_u64().unwrap();
        let kills = path["kills"].as_u64().unwrap();
        let death_count = path["deathCount"].as_u64().unwrap();
        let damaged_hp = path["damagedHp"].as_u64().unwrap();
        let flag_capture = path["flagCapture"].as_u64().unwrap();
        let damaged = path["damaged"].as_u64().unwrap();
        let t_damage_dealt = path["tdamageDealt"].as_u64().unwrap();
        let resource_absorbed = path["resourceAbsorbed"].as_u64().unwrap();
        let credits = path["credits"].as_u64().unwrap();
        let account_dbid = path["accountDBID"].as_u64().unwrap();
        let artillery_fort_equip_damage_dealt = replay[0]["vehicles"][avatar_session_id][0]
            ["artilleryFortEquipDamageDealt"]
            .as_u64()
            .unwrap();
        let no_damage_direct_hits_received = replay[0]["vehicles"][avatar_session_id][0]
            ["noDamageDirectHitsReceived"]
            .as_u64()
            .unwrap();
        let num_defended = path["numDefended"].as_u64().unwrap();
        let stunned = path["stunned"].as_u64().unwrap();
        let equipment_damage_dealt = path["equipmentDamageDealt"].as_u64().unwrap();
        // let is_team_killer = path["isTeamKiller"].as_bool().unwrap();
        // let type_comp_descr = path["typeCompDescr"].as_u64().unwrap();
        let solo_flag_capture = path["soloFlagCapture"].as_u64().unwrap();
        let destructibles_hits = path["destructiblesHits"].as_u64().unwrap();
        // let capturing_base = path["capturingBase"].as_null().unwrap();
        let damage_assisted_stun = path["damageAssistedStun"].as_u64().unwrap();
        let rollouts_count = path["rolloutsCount"].as_u64().unwrap();
        let t_kills = path["tkills"].as_u64().unwrap();
        let potential_damage_received = path["potentialDamageReceived"].as_u64().unwrap();
        let damage_dealt = path["damageDealt"].as_u64().unwrap();
        let destructibles_num_destroyed = replay[0]["vehicles"][avatar_session_id][0]
            ["destructiblesNumDestroyed"]
            .as_u64()
            .unwrap();
        let damage_assisted_smoke = path["damageAssistedSmoke"].as_u64().unwrap();
        let destructibles_damage_dealt = replay[0]["vehicles"][avatar_session_id][0]
            ["destructiblesDamageDealt"]
            .as_u64()
            .unwrap();
        // let flag_actions = path["flagActions"].as_array().unwrap();
        let win_points = path["winPoints"].as_u64().unwrap();
        let explosion_hits = path["explosionHits"].as_u64().unwrap();
        // let team = path["team"].as_u64().unwrap();
        let xp_attack = path["xp/attack"].as_u64().unwrap();
        let t_destroyed_modules = path["tdestroyedModules"].as_u64().unwrap_or(0u64);
        let stun_num = path["stunNum"].as_u64().unwrap();
        let damage_assisted_inspire = path["damageAssistedInspire"].as_u64().unwrap();
        // let in_battle_achievements = path["inBattleAchievements"].as_array().unwrap();
        let achievement_free_xp = path["achievementFreeXP"].as_u64().unwrap();
        let direct_hits = path["directHits"].as_u64().unwrap();

        let opposite_team = if author_team == 1u64 { 2u64 } else { 1u64 };

        let team = if team == "green" {
            author_team
        } else {
            opposite_team
        };
        let mut battles_won = 0;
        let mut battles_lost = 0;
        let mut battles_drew = 0;
        if battle_winner == team {
            battles_won += 1u64;
        } else if [1u64, 2u64].contains(&battle_winner) {
            battles_lost += 1u64;
        } else {
            battles_drew += 1u64;
        };
        let clan_id = replay[0]["players"][&account_dbid.to_string()]["clanDBID"]
            .as_u64()
            .unwrap();

        buf.push(Player {
            account_dbid,
            battles_played: 1u64,
            vehicle_type: vehicle_type.as_str().unwrap().to_string(),
            name: name.to_string(),
            fake_name: fake_name.to_string(),
            wtr,
            battles_survived: if is_alive { 1u64 } else { 0u64 },
            clan_abbrev: clan_abbrev.to_string(),
            avatar_session_id: avatar_session_id.to_string(),
            spotted,
            vehicle_num_captured,
            damage_assisted_track,
            xp_penalty,
            direct_team_hits,
            damage_received,
            life_time,
            piercings_received,
            sniper_damage_dealt,
            piercing_enemy_hits,
            damage_assisted_radio,
            mileage,
            stun_duration,
            piercings,
            damage_blocked_by_armor,
            xp,
            dropped_capture_points,
            // killer_id,
            xp_other,
            // index,
            direct_hits_received,
            damage_received_from_invisibles,
            explosion_hits_received,
            achievement_xp,
            // death_reason,
            capture_points,
            num_recovered,
            direct_enemy_hits,
            // max_health,
            // health,
            // stop_respawn,
            achievement_credits,
            xp_assist,
            shots,
            kills,
            death_count,
            damaged_hp,
            flag_capture,
            damaged,
            t_damage_dealt,
            resource_absorbed,
            credits,
            artillery_fort_equip_damage_dealt,
            no_damage_direct_hits_received,
            num_defended,
            stunned,
            equipment_damage_dealt,
            // is_team_killer,
            // type_comp_descr,
            solo_flag_capture,
            destructibles_hits,
            damage_assisted_stun,
            rollouts_count,
            t_kills,
            potential_damage_received,
            damage_dealt,
            destructibles_num_destroyed,
            damage_assisted_smoke,
            destructibles_damage_dealt,
            win_points,
            explosion_hits,
            xp_attack,
            t_destroyed_modules,
            stun_num,
            damage_assisted_inspire,
            achievement_free_xp,
            direct_hits,
            opposite_team,
            team,
            battles_won,
            battles_lost,
            battles_drew,
            clan_id,
        });
    }
}

fn merge_players(players: &[Player]) -> Vec<Player> {
    let mut merged_players = Vec::new();

    for player in players {
        match merged_players.iter().position(|p| p == player) {
            Some(i) => merged_players[i] += player.clone(),
            None => merged_players.push(player.clone()),
        };
    }

    merged_players
}

fn main() {
    // let args = Args::parse();

    let replays = [
        "20240519_2045_poland-Pl21_CS_63_05_prohorovka.wotreplay",
        "20240519_2300_uk-GB91_Super_Conqueror_10_hills.wotreplay",
    ];
    let mut buf = Vec::new();
    for path in replays {
        let replay = load_replay(&path).unwrap();
        get_players(&replay, &mut buf);
    }

    let merged_players = merge_players(&buf);

    for player in &merged_players {
        println!("{} {}%", player.name, player.survived_pc());
    }
}
