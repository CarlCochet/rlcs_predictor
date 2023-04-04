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

fn parse_data() -> Result<Vec<Match>> {
    let mut file = File::open("data/matches_full.json").expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file.");
    let matches: Vec<Match> = serde_json::from_str(&contents).expect("Error parsing JSON.");
    Ok(matches)
}

fn get_region(regions: &mut Vec<Region>, name: String) -> Option<&mut Region> {
    if let Some(index) = regions.iter().position(|r| r.name == name) {
        return Some(&mut regions[index]);
    }
    None
}

fn get_region_mut(regions: &mut Vec<Region>, name: String) -> Option<&mut Region> {
    if let Some(index) = regions.iter().position(|r| r.name == name) {
        return Some(&mut regions[index]);
    }
    None
}

fn fill_region(regions: &mut Vec<Region>, name: String) {
    if regions.iter().position(|r| r.name == name).is_none() {
        regions.push(Region::new(name.clone()));
    }
}

fn find_players(regions: &Vec<Region>, team: &rlcs_data::Team) -> Option<Vec<Player>> {
    let players_names = team.players.as_ref()?
        .iter()
        .map(|p| p.player.slug.clone())
        .collect::<Vec<String>>();
    let mut result: Vec<Player> = Vec::new();
    for player in players_names {
        let mut found = false;
        for region in &*regions {
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
    Some(result)
}

fn simulate_match(blue_team: &mut Team, orange_team: &mut Team) {
    println!("Double Check.");
}

fn simulate_matches(matches: &Vec<Match>) -> Option<()> {
    let mut regions: Vec<Region> = Vec::new();

    for series in matches {
        let blue_ref: &rlcs_data::Team = match series.blue.as_ref() {
            Some(b) => b,
            None => continue,
        };
        let orange_ref: &rlcs_data::Team = match series.orange.as_ref() {
            Some(o) => o,
            None => continue,
        };
        let blue_players: Vec<Player> = match find_players(&regions, blue_ref) {
            Some(p) => p,
            None => continue,
        };
        let orange_players: Vec<Player> = match find_players(&regions, orange_ref) {
            Some(p) => p,
            None => continue,
        };

        fill_region(&mut regions, series.event.region.clone());
        let region_mut = get_region_mut(&mut regions, series.event.region.clone())?;
        region_mut.fill_team(blue_ref.team.team.name.clone(), &blue_players);
        region_mut.fill_team(orange_ref.team.team.name.clone(), &orange_players);

        let region = get_region(&mut regions, series.event.region.clone())?;
        let (blue_team, orange_team) = region.get_teams_mut(
            &blue_players,
            &orange_players,
        )?;
        println!("Check.");
        simulate_match(blue_team, orange_team);
    }
    Some(())
}

fn main() {
    let matches = match parse_data() {
        Ok(matches) => matches,
        Err(e) => panic!("Error: {}", e),
    };
    simulate_matches(&matches).expect("Error simulating matches.");
    println!("The first event is {}.", matches[0].blue.as_ref().unwrap().team.team.name);
}
