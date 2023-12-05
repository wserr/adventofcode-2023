use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = get_maps()?;

    println!("Result: {:?}", input.iter().min().unwrap());
    Ok(())
}

fn get_maps() -> Result<Vec<usize>> {
    let lines = include_str!("../../assets/day_5/input.txt").lines();

    let result: Vec<Vec<Vec<usize>>> =
        lines
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Vec<Vec<usize>>>, e| {
                let numbers = parse_numbers_from_input(e, 0).unwrap_or_default();

                if acc.is_empty()
                    || (numbers.is_empty() && !acc.last().unwrap_or(&Vec::new()).is_empty())
                {
                    acc.push(Vec::new());
                }

                if !numbers.is_empty() {
                    acc.last_mut().unwrap_or(&mut Vec::new()).push(numbers);
                }
                acc
            });

    let mut seeds: Vec<usize> = Vec::new();

    for (i, map) in result.into_iter().enumerate() {
        if i == 0 {
            seeds = map[0].clone();
        } else {
            let mut new_seeds: Vec<usize> = Vec::new();
            let mut mapped_seeds: Vec<usize> = Vec::new();
            for elements in map {
                let source_start = elements[1];
                let destination_start = elements[0];
                let length = elements[2];

                for seed in seeds.iter() {
                    if *seed >= source_start && *seed <= source_start + length {
                        mapped_seeds.push(*seed);
                        new_seeds.push(seed - source_start + destination_start);
                    }
                }
            }

            for seed in seeds.iter() {
                if !mapped_seeds.contains(seed) {
                    new_seeds.push(*seed);
                }
            }
            seeds = new_seeds;
        }
    }

    Ok(seeds)
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
