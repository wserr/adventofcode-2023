use anyhow::{bail, Result};
use regex::Regex;

fn main() -> Result<()> {
    println!("The result for part 1 is {}", part(r"\d", true)?);
    println!(
        "The result for part 2 is {}",
        part(
            r"\d|oneight|twone|threeight|fiveight|sevenine|nineight|eightwo|one|two|three|four|five|six|seven|eight|nine",
            false
        )?
    );

    Ok(())
}

fn part(regex: &str, is_part_1: bool) -> Result<u32> {
    let input = include_str!("../../assets/day_1/input.txt").lines();
    let mut result: Vec<u32> = Vec::new();
    let re = Regex::new(regex)?;

    for line in input.into_iter() {
        let matches: Vec<&str> = re.find_iter(line).map(|n| n.as_str()).collect();

        let first = matches.first();
        let last = matches.last();

        if let (Some(first), Some(last)) = (first, last) {
            let mut resulting_number = calculate_resulting_number(is_part_1, first, true)?;
            resulting_number.push_str(calculate_resulting_number(is_part_1, last, false)?.as_str());
            result.push(resulting_number.parse::<u32>()?);
        } else {
            bail!("Line could not be calculated: {}", line);
        }
    }
    Ok(result.iter().sum())
}

fn calculate_resulting_number(is_part_1: bool, input: &str, is_first: bool) -> Result<String> {
    if is_part_1 {
        return Ok(input.to_string());
    }
    // if input length is 1, we assume it's a digit
    // if input length is more than 1, we assume the number is written out
    if input.len() == 1 {
        if let Ok(value) = input.parse::<u32>() {
            Ok(value.to_string())
        } else {
            bail!("Value could not be parsed: {:?}", { input });
        }
    } else {
        let result = match input {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "oneight" => {
                if is_first {
                    1
                } else {
                    8
                }
            }
            "twone" => {
                if is_first {
                    2
                } else {
                    1
                }
            }
            "threeight" => {
                if is_first {
                    3
                } else {
                    8
                }
            }
            "fiveight" => {
                if is_first {
                    5
                } else {
                    8
                }
            }
            "sevenine" => {
                if is_first {
                    7
                } else {
                    9
                }
            }
            "eightwo" => {
                if is_first {
                    8
                } else {
                    2
                }
            }
            "nineight" => {
                if is_first {
                    9
                } else {
                    8
                }
            }
            _ => 11,
        };
        if result == 11 {
            bail!("Input could not be matched with a number. Input: {}", input);
        } else {
            Ok(result.to_string())
        }
    }
}
