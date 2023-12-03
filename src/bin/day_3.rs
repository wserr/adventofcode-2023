use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let (symbol_positions, possible_part_numbers) = fetch_schematic_data()?;
    let part_numbers = get_all_part_numbers(&symbol_positions, &possible_part_numbers);
    let result_part_1: usize = part_numbers.into_iter().sum();
    println!("Part 1: {:?}", result_part_1);

    let gear_ratios = get_all_gear_ratios(&symbol_positions, &possible_part_numbers);
    let result_part_2: usize = gear_ratios.into_iter().sum();
    println!("Part 2: {:?}", result_part_2);

    Ok(())
}

fn get_all_part_numbers(
    symbol_positions: &[Symbol],
    possible_part_numbers: &[PartNumber],
) -> Vec<usize> {
    let mut visited_occupied_positions: HashSet<&Vec<Coordinate>> = HashSet::new();
    symbol_positions.iter().fold(Vec::new(), |mut acc, e| {
        let adjacent_coordinates = get_neighbouring_coordinates(&e.position);
        let mut adjacent_part_ids: Vec<usize> = possible_part_numbers
            .iter()
            .filter_map(|p| {
                if visited_occupied_positions.contains(&p.occupied_positions) {
                    return None;
                }
                if p.occupied_positions
                    .iter()
                    .any(|pos| adjacent_coordinates.contains(pos))
                {
                    visited_occupied_positions.insert(&p.occupied_positions);
                    return Some(p.id);
                }
                None
            })
            .collect();
        acc.append(&mut adjacent_part_ids);
        acc
    })
}

fn get_all_gear_ratios(
    symbol_positions: &[Symbol],
    possible_part_numbers: &[PartNumber],
) -> Vec<usize> {
    symbol_positions.iter().fold(Vec::new(), |mut acc, e| {
        let adjacent_coordinates = get_neighbouring_coordinates(&e.position);
        let adjacent_part_ids: Vec<usize> = possible_part_numbers
            .iter()
            .filter_map(|p| {
                if p.occupied_positions
                    .iter()
                    .any(|pos| adjacent_coordinates.contains(pos))
                {
                    return Some(p.id);
                }
                None
            })
            .collect();
        if adjacent_part_ids.len() == 2 && e.symbol == '*' {
            acc.push(adjacent_part_ids[0] * adjacent_part_ids[1])
        }
        acc
    })
}

fn fetch_schematic_data() -> Result<(Vec<Symbol>, Vec<PartNumber>)> {
    let mut symbol_positions: Vec<Symbol> = Vec::new();
    let mut possible_part_numbers: Vec<PartNumber> = Vec::new();

    let lines = include_str!("../../assets/day_3/input.txt").lines();

    for (i1, line) in lines.enumerate() {
        let mut constructing_number = false;
        let mut number = String::new();
        let mut occupied_positions: Vec<Coordinate> = Vec::new();
        let line_length = line.chars().count();
        for (i2, element) in line.chars().enumerate() {
            let mut save_number = false;
            if element.is_numeric() {
                constructing_number = true;
                number.push(element);
                occupied_positions.push(Coordinate { x: i2, y: i1 });
            } else if element == '.' {
                save_number = constructing_number
            } else {
                symbol_positions.push(Symbol {
                    symbol: element,
                    position: Coordinate { x: i2, y: i1 },
                });
                save_number = constructing_number;
            }
            if i2 == line_length - 1 && constructing_number {
                save_number = true;
            }
            if save_number {
                constructing_number = false;
                let id = number.parse::<usize>()?;
                possible_part_numbers.push(PartNumber {
                    id,
                    occupied_positions,
                });
                number = String::new();
                occupied_positions = Vec::new();
            }
        }
    }
    Ok((symbol_positions, possible_part_numbers))
}

#[derive(Debug)]
pub struct PartNumber {
    id: usize,
    occupied_positions: Vec<Coordinate>,
}

pub struct Symbol {
    symbol: char,
    position: Coordinate,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

fn get_neighbouring_coordinates(input: &Coordinate) -> Vec<Coordinate> {
    let mut result = Vec::new();

    if input.x != 0 && input.y != 0 {
        result.push(Coordinate {
            x: input.x - 1,
            y: input.y - 1,
        });
    }
    if input.x != 0 {
        result.push(Coordinate {
            x: input.x - 1,
            y: input.y,
        });
        result.push(Coordinate {
            x: input.x - 1,
            y: input.y + 1,
        });
    }
    if input.y != 0 {
        result.push(Coordinate {
            x: input.x,
            y: input.y - 1,
        });
        result.push(Coordinate {
            x: input.x + 1,
            y: input.y - 1,
        });
    }

    result.push(Coordinate {
        x: input.x + 1,
        y: input.y,
    });

    result.push(Coordinate {
        x: input.x,
        y: input.y + 1,
    });

    result.push(Coordinate {
        x: input.x + 1,
        y: input.y + 1,
    });

    result
}
