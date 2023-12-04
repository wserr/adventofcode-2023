use std::collections::HashMap;

use anyhow::{bail, Result};
use regex::Regex;

fn main() -> Result<()> {
    let sum_part_1 = calculate_part_1()?;
    println!("Result part 1: {}", sum_part_1);

    let sum_part_2 = calculate_part_2()?;

    println!("Result part 2: {}", sum_part_2);
    Ok(())
}

fn calculate_part_2() -> Result<usize> {
    let lines = include_str!("../../assets/day_4/input.txt").lines();

    let mut result: HashMap<usize, usize> = HashMap::new();

    for (index, line) in lines.enumerate() {
        result.entry(index).and_modify(|e| *e += 1).or_insert(1);
        let amount = *result.get(&index).unwrap_or(&0_usize);
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            bail!("Input string was not in correct format: {}", line);
        }

        // Skip first number from first part, this is the card number
        let winning_numbers = parse_numbers_from_input(parts[0], 1)?;
        let your_numbers = parse_numbers_from_input(parts[1], 0)?;

        let multiplier: usize = your_numbers.into_iter().fold(0, |acc, e| {
            if winning_numbers.contains(&e) {
                return acc + 1;
            }
            acc
        });

        for i in 1..(multiplier + 1) {
            result
                .entry(index + i)
                .and_modify(|e| *e += amount)
                .or_insert(amount);
        }
    }
    Ok(result.into_iter().fold(0, |acc, e| acc + e.1))
}

fn calculate_part_1() -> Result<usize> {
    let mut sum: usize = 0;
    let lines = include_str!("../../assets/day_4/input.txt").lines();
    for line in lines.into_iter() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            bail!("Input string was not in correct format: {}", line);
        }

        // Skip first number from first part, this is the card number
        let winning_numbers = parse_numbers_from_input(parts[0], 1)?;
        let your_numbers = parse_numbers_from_input(parts[1], 0)?;

        let multiplier: usize = your_numbers.into_iter().fold(0, |acc, e| {
            if winning_numbers.contains(&e) {
                return acc + 1;
            }
            acc
        });

        match multiplier {
            0 => (),
            1 => {
                sum += 1;
            }
            value => {
                sum += 2_usize.pow((value - 1) as u32);
            }
        };
    }
    Ok(sum)
}

fn parse_numbers_from_input(input: &str, skip: usize) -> Result<Vec<usize>> {
    let number_matcher = Regex::new(r"\d+")?;
    Ok(number_matcher
        .find_iter(input)
        .skip(skip)
        .map(|m| m.as_str().parse::<usize>())
        .filter_map(|result| match result {
            Ok(value) => Some(value),
            _ => None,
        })
        .collect())
}
