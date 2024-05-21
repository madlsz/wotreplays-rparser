use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops;

#[derive(Clone)]
pub struct Player {
    pub account_dbid: u64,
    pub battles_played: u64,
    pub vehicle_type: String,
    pub name: String,
    pub fake_name: String,
    pub wtr: u64,
    pub battles_survived: u64,
    pub clan_abbrev: String,
    pub avatar_session_id: String,
    pub spotted: u64,
    pub vehicle_num_captured: u64,
    pub damage_assisted_track: u64,
    pub xp_penalty: u64,
    pub direct_team_hits: u64,
    pub damage_received: u64,
    pub life_time: u64,
    pub piercings_received: u64,
    pub sniper_damage_dealt: u64,
    pub piercing_enemy_hits: u64,
    pub damage_assisted_radio: u64,
    pub mileage: u64,
    pub stun_duration: f64,
    pub piercings: u64,
    pub damage_blocked_by_armor: u64,
    pub xp: u64,
    pub dropped_capture_points: u64,
    // pub killer_id: u64,
    pub xp_other: u64,
    // pub index: u64,
    pub direct_hits_received: u64,
    pub damage_received_from_invisibles: u64,
    pub explosion_hits_received: u64,
    pub achievement_xp: u64,
    // pub death_reason: i64,
    pub capture_points: u64,
    pub num_recovered: u64,
    pub direct_enemy_hits: u64,
    // pub max_health: u64,
    // pub health: u64,
    // pub stop_respawn: bool,
    pub achievement_credits: u64,
    pub xp_assist: u64,
    pub shots: u64,
    pub kills: u64,
    pub death_count: u64,
    pub damaged_hp: u64,
    pub flag_capture: u64,
    pub damaged: u64,
    pub t_damage_dealt: u64,
    pub resource_absorbed: u64,
    pub credits: u64,
    pub artillery_fort_equip_damage_dealt: u64,
    pub no_damage_direct_hits_received: u64,
    pub num_defended: u64,
    pub stunned: u64,
    pub equipment_damage_dealt: u64,
    // pub is_team_killer: bool,
    // pub type_comp_descr: u64,
    pub solo_flag_capture: u64,
    pub destructibles_hits: u64,
    pub damage_assisted_stun: u64,
    pub rollouts_count: u64,
    pub t_kills: u64,
    pub potential_damage_received: u64,
    pub damage_dealt: u64,
    pub destructibles_num_destroyed: u64,
    pub damage_assisted_smoke: u64,
    pub destructibles_damage_dealt: u64,
    pub win_points: u64,
    pub explosion_hits: u64,
    pub xp_attack: u64,
    pub t_destroyed_modules: u64,
    pub stun_num: u64,
    pub damage_assisted_inspire: u64,
    pub achievement_free_xp: u64,
    pub direct_hits: u64,
    pub opposite_team: u64,
    pub team: u64,
    pub battles_won: u64,
    pub battles_lost: u64,
    pub battles_drew: u64,
    pub clan_id: u64,
    pub stats: HashMap<String, fn(&Player) -> Stats>,
}

pub enum Stats {
    F64(f64),
    U64(u64),
    String(String),
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stats::F64(x) => write!(f, "{}", x.to_string()),
            Stats::U64(x) => write!(f, "{}", x.to_string()),
            Stats::String(x) => write!(f, "{x}"),
        }
    }
}

impl Player {
    pub fn create_stats_pointers(&mut self) {
        self.stats.insert(String::from("name"), Player::name);
        self.stats
            .insert(String::from("account_dbid"), Player::account_dbid);
        self.stats
            .insert(String::from("damage_dealt_pb"), Player::damage_dealt_pb);
        self.stats
            .insert(String::from("survived_pc"), Player::survived_pc);
        self.stats.insert(String::from("wr_pc"), Player::wr_pc);
        self.stats
            .insert(String::from("frags_pb"), Player::frags_pb);
        self.stats.insert(String::from("xp_pb"), Player::xp_pb);
        self.stats
            .insert(String::from("vehicle_shots_pb"), Player::vehicle_shots_pb);
        self.stats
            .insert(String::from("vehicle_hits_pb"), Player::vehicle_hits_pb);
        self.stats.insert(
            String::from("vehicle_explosion_hits_pb"),
            Player::vehicle_explosion_hits_pb,
        );
        self.stats.insert(
            String::from("vehicle_pierced_pb"),
            Player::vehicle_pierced_pb,
        );
        self.stats.insert(
            String::from("sniper_damage_dealt_pb"),
            Player::sniper_damage_dealt_pb,
        );
        self.stats.insert(
            String::from("vehicle_incoming_hits_pb"),
            Player::vehicle_incoming_hits_pb,
        );
        self.stats.insert(
            String::from("pierced_received_pb"),
            Player::pierced_received_pb,
        );
        self.stats.insert(
            String::from("explosion_hits_received_pb"),
            Player::explosion_hits_received_pb,
        );
        self.stats.insert(
            String::from("no_damage_direct_hits_received_pb"),
            Player::no_damage_direct_hits_received_pb,
        );
        self.stats.insert(
            String::from("potential_damage_received_pb"),
            Player::potential_damage_received_pb,
        );
        self.stats.insert(
            String::from("damage_blocked_by_armor_pb"),
            Player::damage_blocked_by_armor_pb,
        );
        self.stats
            .insert(String::from("spotted_pb"), Player::spotted_pb);
        self.stats
            .insert(String::from("damaged_pb"), Player::damaged_pb);
        self.stats
            .insert(String::from("killed_pb"), Player::killed_pb);
        self.stats.insert(
            String::from("damage_assisted_pb"),
            Player::damage_assisted_pb,
        );
        self.stats.insert(
            String::from("damage_assisted_stun_pb"),
            Player::damage_assisted_stun_pb,
        );
        self.stats
            .insert(String::from("stun_num_pb"), Player::stun_num_pb);
        self.stats
            .insert(String::from("capture_points_pb"), Player::capture_points_pb);
        self.stats
            .insert(String::from("defence_points_pb"), Player::defence_points_pb);
        self.stats
            .insert(String::from("mileage_pb"), Player::mileage_pb);
        self.stats
            .insert(String::from("accuracy_pc"), Player::accuracy_pc);
        self.stats
            .insert(String::from("penetration_pc"), Player::penetration_pc);
    }

    fn name(&self) -> Stats {
        Stats::String(self.name.clone())
    }

    fn account_dbid(&self) -> Stats {
        Stats::U64(self.account_dbid)
    }

    fn survived_pc(&self) -> Stats {
        Stats::F64(self.battles_survived as f64 / self.battles_played as f64 * 100f64)
    }

    fn wr_pc(&self) -> Stats {
        Stats::F64(self.battles_won as f64 / self.battles_played as f64 * 100f64)
    }

    fn frags_pb(&self) -> Stats {
        Stats::F64(self.kills as f64 / self.battles_played as f64)
    }

    fn xp_pb(&self) -> Stats {
        Stats::F64(self.xp as f64 / self.battles_played as f64)
    }

    fn vehicle_shots_pb(&self) -> Stats {
        Stats::F64(self.shots as f64 / self.battles_played as f64)
    }

    fn vehicle_hits_pb(&self) -> Stats {
        Stats::F64(self.direct_hits as f64 / self.battles_played as f64)
    }

    fn vehicle_explosion_hits_pb(&self) -> Stats {
        Stats::F64(self.explosion_hits as f64 / self.battles_played as f64)
    }

    fn vehicle_pierced_pb(&self) -> Stats {
        Stats::F64(self.piercing_enemy_hits as f64 / self.battles_played as f64)
    }

    fn damage_dealt_pb(&self) -> Stats {
        Stats::F64(self.damage_dealt as f64 / self.battles_played as f64)
    }

    fn sniper_damage_dealt_pb(&self) -> Stats {
        Stats::F64(self.sniper_damage_dealt as f64 / self.battles_played as f64)
    }

    fn vehicle_incoming_hits_pb(&self) -> Stats {
        Stats::F64(self.direct_hits_received as f64 / self.battles_played as f64)
    }

    fn pierced_received_pb(&self) -> Stats {
        Stats::F64(self.piercings_received as f64 / self.battles_played as f64)
    }

    fn explosion_hits_received_pb(&self) -> Stats {
        Stats::F64(self.explosion_hits_received as f64 / self.battles_played as f64)
    }

    fn no_damage_direct_hits_received_pb(&self) -> Stats {
        Stats::F64(self.no_damage_direct_hits_received as f64 / self.battles_played as f64)
    }

    fn potential_damage_received_pb(&self) -> Stats {
        Stats::F64(self.potential_damage_received as f64 / self.battles_played as f64)
    }

    fn damage_blocked_by_armor_pb(&self) -> Stats {
        Stats::F64(self.damage_blocked_by_armor as f64 / self.battles_played as f64)
    }

    fn spotted_pb(&self) -> Stats {
        Stats::F64(self.spotted as f64 / self.battles_played as f64)
    }

    fn damaged_pb(&self) -> Stats {
        Stats::F64(self.damaged as f64 / self.battles_played as f64)
    }

    fn killed_pb(&self) -> Stats {
        Stats::F64(self.kills as f64 / self.battles_played as f64)
    }

    /// radio + track + stun
    fn damage_assisted_pb(&self) -> Stats {
        Stats::F64(
            (self.damage_assisted_radio + self.damage_assisted_track + self.damage_assisted_stun)
                as f64
                / self.battles_played as f64,
        )
    }

    fn damage_assisted_stun_pb(&self) -> Stats {
        Stats::F64(self.damage_assisted_stun as f64 / self.battles_played as f64)
    }

    fn stun_num_pb(&self) -> Stats {
        Stats::F64(self.stun_num as f64 / self.battles_played as f64)
    }

    fn capture_points_pb(&self) -> Stats {
        Stats::F64(self.capture_points as f64 / self.battles_played as f64)
    }

    fn defence_points_pb(&self) -> Stats {
        Stats::F64(self.dropped_capture_points as f64 / self.battles_played as f64)
    }

    fn mileage_pb(&self) -> Stats {
        Stats::F64(self.mileage as f64 / self.battles_played as f64)
    }

    fn accuracy_pc(&self) -> Stats {
        match self.direct_hits {
            0u64 => Stats::F64(0f64),
            _ => Stats::F64(self.direct_hits as f64 / self.shots as f64 * 100f64),
        }
    }

    fn penetration_pc(&self) -> Stats {
        match self.piercing_enemy_hits {
            0u64 => Stats::F64(0f64),
            _ => Stats::F64(self.piercing_enemy_hits as f64 / self.direct_hits as f64 * 100f64),
        }
    }
}

impl cmp::PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.account_dbid == other.account_dbid
    }
}

impl ops::AddAssign for Player {
    fn add_assign(&mut self, rhs: Self) {
        self.battles_played += rhs.battles_played;
        self.battles_survived += rhs.battles_survived;
        self.spotted += rhs.spotted;
        self.vehicle_num_captured += rhs.vehicle_num_captured;
        self.damage_assisted_track += rhs.damage_assisted_track;
        self.xp_penalty += rhs.xp_penalty;
        self.direct_team_hits += rhs.direct_team_hits;
        self.damage_received += rhs.damage_received;
        self.life_time += rhs.life_time;
        self.piercings_received += rhs.piercings_received;
        self.sniper_damage_dealt += rhs.sniper_damage_dealt;
        self.piercing_enemy_hits += rhs.piercing_enemy_hits;
        self.damage_assisted_radio += rhs.damage_assisted_radio;
        self.mileage += rhs.mileage;
        self.stun_duration += rhs.stun_duration;
        self.piercings += rhs.piercings;
        self.damage_blocked_by_armor += rhs.damage_blocked_by_armor;
        self.xp += rhs.xp;
        self.dropped_capture_points += rhs.dropped_capture_points;
        self.xp_other += rhs.xp_other;
        self.direct_hits_received += rhs.direct_hits_received;
        self.damage_received_from_invisibles += rhs.damage_received_from_invisibles;
        self.explosion_hits_received += rhs.explosion_hits_received;
        self.achievement_xp += rhs.achievement_xp;
        self.capture_points += rhs.capture_points;
        self.num_recovered += rhs.num_recovered;
        self.direct_enemy_hits += rhs.direct_enemy_hits;
        self.achievement_credits += rhs.achievement_credits;
        self.xp_assist += rhs.xp_assist;
        self.shots += rhs.shots;
        self.kills += rhs.kills;
        self.death_count += rhs.death_count;
        self.damaged_hp += rhs.damaged_hp;
        self.flag_capture += rhs.flag_capture;
        self.damaged += rhs.damaged;
        self.t_damage_dealt += rhs.t_damage_dealt;
        self.resource_absorbed += rhs.resource_absorbed;
        self.credits += rhs.credits;
        self.artillery_fort_equip_damage_dealt += rhs.artillery_fort_equip_damage_dealt;
        self.no_damage_direct_hits_received += rhs.no_damage_direct_hits_received;
        self.num_defended += rhs.num_defended;
        self.stunned += rhs.stunned;
        self.equipment_damage_dealt += rhs.equipment_damage_dealt;
        self.solo_flag_capture += rhs.solo_flag_capture;
        self.destructibles_hits += rhs.destructibles_hits;
        self.damage_assisted_stun += rhs.damage_assisted_stun;
        self.rollouts_count += rhs.rollouts_count;
        self.t_kills += rhs.t_kills;
        self.potential_damage_received += rhs.potential_damage_received;
        self.damage_dealt += rhs.damage_dealt;
        self.destructibles_num_destroyed += rhs.destructibles_num_destroyed;
        self.damage_assisted_smoke += rhs.damage_assisted_smoke;
        self.destructibles_damage_dealt += rhs.destructibles_damage_dealt;
        self.win_points += rhs.win_points;
        self.explosion_hits += rhs.explosion_hits;
        self.xp_attack += rhs.xp_attack;
        self.t_destroyed_modules += rhs.t_destroyed_modules;
        self.stun_num += rhs.stun_num;
        self.damage_assisted_inspire += rhs.damage_assisted_inspire;
        self.achievement_free_xp += rhs.achievement_free_xp;
        self.direct_hits += rhs.direct_hits;
        self.battles_won += rhs.battles_won;
        self.battles_lost += rhs.battles_lost;
        self.battles_drew += rhs.battles_drew;
    }
}
