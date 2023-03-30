use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct RLCSData {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub _id: String,
    pub slug: String,
    pub octane_id: Option<String>,
    pub event: Event,
    pub stage: Stage,
    pub date: String,
    pub format: Format,
    pub blue: Option<Team>,
    pub orange: Option<Team>,
    pub number: i32,
    pub games: Option<Vec<Game>>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub _id: String,
    pub slug: String,
    pub name: String,
    pub region: String,
    pub mode: i32,
    pub tier: String,
    pub image: String,
    pub groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub _id: i32,
    pub name: String,
    pub format: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Format {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub length: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub team: TeamData,
    pub players: Option<Vec<Player>>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamData {
    pub team: TeamInfo,
    pub stats: Option<Stats>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamInfo {
    pub _id: String,
    pub slug: Option<String>,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub player: PlayerInfo,
    pub stats: Stats,
    pub advanced: Advanced,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub _id: String,
    pub slug: String,
    pub tag: String,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub _id: Option<String>,
    pub blue: i32,
    pub orange: i32,
    pub duration: Option<i32>,
    pub ballchasing: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub core: StatsCore,
    pub boost: Option<StatsBoost>,
    pub movement: Option<StatsMovement>,
    pub positioning: Option<StatsPositioning>,
    pub demo: Option<StatsDemo>,
}

#[derive(Serialize, Deserialize)]
pub struct StatsCore {
    pub shots: i32,
    pub goals: i32,
    pub saves: i32,
    pub assists: i32,
    pub score: i32,
    #[serde(rename(serialize = "shootingPercentage", deserialize = "shootingPercentage"))]
    pub shooting_percentage: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsBoost {
    pub bpm: f32,
    pub bcpm: f32,
    #[serde(rename(serialize = "avgAmount", deserialize = "avgAmount"))]
    pub avg_amount: f32,
    #[serde(rename(serialize = "amountCollected", deserialize = "amountCollected"))]
    pub amount_collected: f32,
    #[serde(rename(serialize = "amountStolen", deserialize = "amountStolen"))]
    pub amount_stolen: f32,
    #[serde(rename(serialize = "amountCollectedBig", deserialize = "amountCollectedBig"))]
    pub amount_collected_big: i32,
    #[serde(rename(serialize = "amountStolenBig", deserialize = "amountStolenBig"))]
    pub amount_stolen_big: i32,
    #[serde(rename(serialize = "amountCollectedSmall", deserialize = "amountCollectedSmall"))]
    pub amount_collected_small: i32,
    #[serde(rename(serialize = "amountStolenSmall", deserialize = "amountStolenSmall"))]
    pub amount_stolen_small: i32,
    #[serde(rename(serialize = "countCollectedBig", deserialize = "countCollectedBig"))]
    pub count_collected_big: i32,
    #[serde(rename(serialize = "countStolenBig", deserialize = "countStolenBig"))]
    pub count_stolen_big: i32,
    #[serde(rename(serialize = "countCollectedSmall", deserialize = "countCollectedSmall"))]
    pub count_collected_small: i32,
    #[serde(rename(serialize = "countStolenSmall", deserialize = "countStolenSmall"))]
    pub count_stolen_small: i32,
    #[serde(rename(serialize = "amountOverfill", deserialize = "amountOverfill"))]
    pub amount_overfill: i32,
    #[serde(rename(serialize = "amountOverfillStolen", deserialize = "amountOverfillStolen"))]
    pub amount_overfill_stolen: i32,
    #[serde(rename(serialize = "amountUsedWhileSupersonic", deserialize = "amountUsedWhileSupersonic"))]
    pub amount_used_while_supersonic: i32,
    #[serde(rename(serialize = "timeZeroBoost", deserialize = "timeZeroBoost"))]
    pub time_zero_boost: f32,
    #[serde(rename(serialize = "timeFullBoost", deserialize = "timeFullBoost"))]
    pub time_full_boost: f32,
    #[serde(rename(serialize = "timeBoost0To25", deserialize = "timeBoost0To25"))]
    pub time_boost_0_25: f32,
    #[serde(rename(serialize = "timeBoost25To50", deserialize = "timeBoost25To50"))]
    pub time_boost_25_50: f32,
    #[serde(rename(serialize = "timeBoost50To75", deserialize = "timeBoost50To75"))]
    pub time_boost_50_75: f32,
    #[serde(rename(serialize = "timeBoost75To100", deserialize = "timeBoost75To100"))]
    pub time_boost_75_100: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsMovement {
    #[serde(rename(serialize = "totalDistance", deserialize = "totalDistance"))]
    pub total_distance: f32,
    #[serde(rename(serialize = "timeSupersonicSpeed", deserialize = "timeSupersonicSpeed"))]
    pub time_supersonic_speed: f32,
    #[serde(rename(serialize = "timeBoostSpeed", deserialize = "timeBoostSpeed"))]
    pub time_boost_speed: f32,
    #[serde(rename(serialize = "timeSlowSpeed", deserialize = "timeSlowSpeed"))]
    pub time_slow_speed: f32,
    #[serde(rename(serialize = "timeGround", deserialize = "timeGround"))]
    pub time_ground: f32,
    #[serde(rename(serialize = "timeLowAir", deserialize = "timeLowAir"))]
    pub time_low_air: f32,
    #[serde(rename(serialize = "timeHighAir", deserialize = "timeHighAir"))]
    pub time_high_air: f32,
    #[serde(rename(serialize = "timePowerslide", deserialize = "timePowerslide"))]
    pub time_powerslide: f32,
    #[serde(rename(serialize = "countPowerslide", deserialize = "countPowerslide"))]
    pub count_powerslide: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsPositioning {
    #[serde(rename(serialize = "timeDefensiveThird", deserialize = "timeDefensiveThird"))]
    pub time_defensive_third: f32,
    #[serde(rename(serialize = "timeNeutralThird", deserialize = "timeNeutralThird"))]
    pub time_neutral_third: f32,
    #[serde(rename(serialize = "timeOffensiveThird", deserialize = "timeOffensiveThird"))]
    pub time_offensive_third: f32,
    #[serde(rename(serialize = "timeDefensiveHalf", deserialize = "timeDefensiveHalf"))]
    pub time_defensive_half: f32,
    #[serde(rename(serialize = "timeOffensiveHalf", deserialize = "timeOffensiveHalf"))]
    pub time_offensive_half: f32,
    #[serde(rename(serialize = "timeBehindBall", deserialize = "timeBehindBall"))]
    pub time_behind_ball: f32,
    #[serde(rename(serialize = "timeInfrontBall", deserialize = "timeInfrontBall"))]
    pub time_infront_ball: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StatsDemo {
    pub inflicted: i32,
    pub taken: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Advanced {
    #[serde(rename(serialize = "goalParticipation", deserialize = "goalParticipation"))]
    pub goal_participation: f32,
    pub rating: f32,
}

fn default_format() -> Format {
    Format {
        kind: "N/A".to_string(),
        length: 0,
    }
}