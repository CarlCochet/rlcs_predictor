use kdam::tqdm;
use glicko_2::{Rating, Tuning, game};

use crate::player::Player;
use crate::team::Team;
use crate::region::Region;
use crate::rlcs_data;

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

fn simulate_match(blue_team: &mut Team, orange_team: &mut Team, series: &rlcs_data::Match) -> Option<()> {
    let mut blue_score = 0;
    let mut orange_score = 0;
    let games = series.games.clone()?;

    for game in &games {
        let blue_scored = game.blue;
        let orange_scored = game.orange;
        if blue_scored > orange_scored {
            blue_score += 1;
        } else {
            orange_score += 1;
        }
    }

    let blue_tuning = Tuning::new(blue_team.rating.mu, blue_team.rating.phi, blue_team.rating.sigma, blue_team.rating.tau);
    let orange_tuning = Tuning::new(orange_team.rating.mu, orange_team.rating.phi, orange_team.rating.sigma, orange_team.rating.tau);
    let mut blue_rating = Rating::new(&blue_tuning);
    let mut orange_rating = Rating::new(&orange_tuning);

    if blue_score > orange_score {
        game::compete(&mut blue_rating, &mut orange_rating, false);
    } else {
        game::compete(&mut orange_rating, &mut blue_rating, false);
    }

    blue_team.rating.update(blue_rating.mu, blue_rating.phi, blue_rating.sigma, blue_rating.is_scaled);
    orange_team.rating.update(orange_rating.mu, orange_rating.phi, orange_rating.sigma, orange_rating.is_scaled);

    Some(())
}

pub fn simulate_matches(matches: &Vec<rlcs_data::Match>) -> Option<Vec<Region>> {
    let mut regions: Vec<Region> = Vec::new();

    for series in tqdm!(matches.iter()) {
        let blue_ref: &rlcs_data::Team = match series.blue.as_ref() {
            Some(b) => b,
            None => continue,
        };
        let orange_ref: &rlcs_data::Team = match series.orange.as_ref() {
            Some(o) => o,
            None => continue,
        };
        if blue_ref.players.is_none() || orange_ref.players.is_none() {
            continue;
        }
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
        match simulate_match(blue_team, orange_team, series) {
            Some(_) => (),
            None => continue,
        };
    }
    Some(regions)
}