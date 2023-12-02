use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;

pub fn main() -> Result<()> {
    println!("Result of part 1 is {}", part_1()?);
    println!("Result of part 2 is {}", part_2()?);

    Ok(())
}

fn part_1() -> Result<u32> {
    let possible_game_ids: Vec<u32> = fetch_game_stats()?
        .iter()
        .filter_map(|g| {
            if g.1.red <= 12 && g.1.green <= 13 && g.1.blue <= 14 {
                return Some(*g.0);
            }
            None
        })
        .collect();

    Ok(possible_game_ids.iter().sum())
}

fn part_2() -> Result<u32> {
    let possible_game_ids: Vec<u32> = fetch_game_stats()?
        .iter()
        .map(|g| g.1.red * g.1.green * g.1.blue)
        .collect();

    Ok(possible_game_ids.iter().sum())
}

fn fetch_game_stats() -> Result<HashMap<u32, GameStats>> {
    let mut result: HashMap<u32, GameStats> = HashMap::new();
    let input = include_str!("../../assets/day_2/input.txt").lines();

    for line in input.into_iter() {
        let game_id = get_game_id(line)?;
        let green = find_max_color_pick(line, Colors::Green)?;
        let red = find_max_color_pick(line, Colors::Red)?;
        let blue = find_max_color_pick(line, Colors::Blue)?;

        result.insert(game_id, GameStats { green, red, blue });
    }

    Ok(result)
}

fn find_max_color_pick(line: &str, color: Colors) -> Result<u32> {
    let regex = match color {
        Colors::Red => Regex::new(r"(\d+) red")?,
        Colors::Green => Regex::new(r"(\d+) green")?,
        Colors::Blue => Regex::new(r"(\d+) blue")?,
    };

    let mut result: Vec<u32> = Vec::new();

    for (_, [color_amount]) in regex.captures_iter(line).map(|caps| caps.extract()) {
        result.push(color_amount.parse()?);
    }
    match result.iter().max() {
        Some(max) => Ok(*max),
        None => Err(anyhow::anyhow!(
            "No color {:?} found in line {}",
            color,
            line
        )),
    }
}

fn get_game_id(line: &str) -> Result<u32> {
    let game_regex = Regex::new(r"Game (\d+)")?;
    let mut result: Vec<u32> = Vec::new();

    for (_, [game_id]) in game_regex.captures_iter(line).map(|caps| caps.extract()) {
        result.push(game_id.parse()?);
    }
    match result.first() {
        Some(result) => Ok(*result),
        None => Err(anyhow::anyhow!("Game id not found in line {}", line)),
    }
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct GameStats {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_id() {
        let result = get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(1, result.unwrap());
    }

    #[test]
    fn test_get_color_red() {
        let result = find_max_color_pick(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Colors::Green,
        );
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn test_get_color_blue() {
        let result = find_max_color_pick(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Colors::Blue,
        );
        assert_eq!(6, result.unwrap());
    }
}
