mod player;
mod region;
mod team;
mod rlcs_data;

use std::fs::File;
use std::io::prelude::*;
use serde_json::Result;

use crate::player::Player;
use crate::team::Team;
use crate::region::Region;
use crate::rlcs_data::Match;

fn parse_data() -> Result<(Vec<Match>)> {
    let mut file = File::open("data/matches_full.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let matches: Vec<Match> = serde_json::from_str(&contents)?;
    Ok(matches)
}

fn find_region(regions: &mut Vec<Region>, name: String) -> Option<&mut Region> {
    regions
        .iter_mut()
        .find(|r| r.name == name)
        .or_else(|| {
            regions.push(Region::new(name.clone()));
            regions.last_mut()
        })
}

fn find_players(regions: &mut Vec<Region>, players: &Vec<String>) -> Vec<Player> {
    let mut result: Vec<Player> = Vec::new();
    for player in players {
        let mut found = false;
        for region in regions {
            if let Some(p) = region.find_player(player.clone()) {
                result.push(p.clone());
                found = true;
                break;
            }
        }
        if !found {
            result.push(Player::new(player.clone()));
        }
    }
    result
}

fn simulate_matches(matches: &Vec<Match>) {
    let mut regions: Vec<Region> = Vec::new();

    for series in matches {
        let region = find_region(&mut regions, series.event.region.clone())?;
        let blue_players: Vec<Player> = series.blue.as_ref().unwrap()
            .players.as_ref().unwrap()
            .iter()
            .map(|p| Player::new(p.name.clone()))
            .collect();
        let blue_team = region.find_team(series.blue.as_ref().unwrap().team.team.name.clone())?;
        let orange_team = region.find_team(series.orange.as_ref().unwrap().team.team.name.clone())?;
    }
}

fn main() {
    let matches = match parse_data() {
        Ok(matches) => matches,
        Err(e) => panic!("Error: {}", e),
    };
    simulate_matches(&matches);
    println!("The first event is {}.", matches[0].blue.as_ref().unwrap().team.team.name);
}
