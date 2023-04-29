mod player;
mod region;
mod team;
mod rlcs_data;
mod rating_params;
mod simulator;

use std::fs::File;
use std::io::prelude::*;
use serde_json::Result;

use crate::rlcs_data::Match;
use crate::simulator::simulate_matches;
use crate::region::Region;

fn parse_data() -> Result<Vec<Match>> {
    let mut file = File::open("data/matches_full.json").expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file.");
    let matches: Vec<Match> = serde_json::from_str(&contents).expect("Error parsing JSON.");
    Ok(matches)
}

fn display_ratings(regions: Vec<Region>) {
    for region in regions {
        println!("{}:", region.name);
        for team in region.teams {
            for player in team.players {
                if player.games_played > 1 {
                    println!("{}: {}. Games played: {}", player.name, player.rating, player.games_played);
                }
            }
        }
        println!();
    }
}

fn main() {
    let matches = match parse_data() {
        Ok(matches) => matches,
        Err(e) => panic!("Error: {}", e),
    };
    match simulate_matches(&matches) {
        Some(regions) => display_ratings(regions),
        None => print!("\nError simulating the matches."),
    };

}
