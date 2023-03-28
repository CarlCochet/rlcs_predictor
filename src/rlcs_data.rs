use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RLCSData {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    _id: String,
    slug: String,
    octane_id: String,
    event: Event,
    stage: Stage,
    date: String,
    format: Format,
    blue: Team,
    orange: Team,
    number: i32,
    games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    _id: String,
    slug: String,
    name: String,
    region: String,
    mode: i32,
    tier: String,
    image: String,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Stage {
    _id: i32,
    name: String,
    format: String,
}

#[derive(Serialize, Deserialize)]
pub struct Format {
    _type: String,
    length: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    team_info: TeamInfo,
    stats: Stats,
    players: Vec<Player>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo {
    _id: String,
    slug: String,
    name: String,
    image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    player_info: PlayerInfo,
    stats: Stats,
    advanced: Advanced,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    _id: String,
    slug: String,
    name: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    _id: String,
    blue: i32,
    orange: i32,
    duration: i32,
    ballchasing: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    core: StatsCore,
    boost: StatsBoost,
    movement: StatsMovement,
    positioning: StatsPositioning,
    demo: StatsDemo,
}

#[derive(Serialize, Deserialize)]
pub struct StatsCore {
    shots: i32,
    goals: i32,
    saves: i32,
    assists: i32,
    score: i32,
    shooting_percentage: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsBoost {
    bpm: f32,
    bcpm: f32,
    avg_amount: f32,
    amount_collected: f32,
    amount_stolen: f32,
    amount_collected_big: i32,
    amount_stolen_big: i32,
    amount_collected_small: i32,
    amount_stolen_small: i32,
    count_collected_big: i32,
    count_stolen_big: i32,
    count_collected_small: i32,
    count_stolen_small: i32,
    amount_overfill: i32,
    amount_overfill_stolen: i32,
    amount_used_while_supersonic: i32,
    time_zero_boost: f32,
    time_full_boost: f32,
    time_boost_0_25: f32,
    time_boost_25_50: f32,
    time_boost_50_75: f32,
    time_boost_75_100: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsMovement {
    total_distance: f32,
    time_supersonic_speed: f32,
    time_boost_speed: f32,
    time_slow_speed: f32,
    time_ground: f32,
    time_low_air: f32,
    time_high_air: f32,
    time_powerslide: f32,
    count_powerslide: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsPositioning {
    time_defensive_third: f32,
    time_neutral_third: f32,
    time_offensive_third: f32,
    time_defensive_half: f32,
    time_offensive_half: f32,
    time_behind_ball: f32,
    time_infront_ball: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDemo {
    inflicted: i32,
    taken: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Advanced {
    goal_participation: f32,
    rating: f32,
}
