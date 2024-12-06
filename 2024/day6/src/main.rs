use std::{fs::read_to_string, ops::Index};
use regex::Regex;
use anyhow::Result;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

fn main() -> Result<()> {
    let (map, width, height, x, y) = load_map().unwrap();
    let visits = search(x, y, &width, &height, map)?;
    // let needs_search = visits.clone();

    dbg!("fell out of walking with ? visits", visits.len());
    Ok(())
}

fn load_map() -> Result<(Vec<Vec<char>>, i32, i32, i32, i32)> {
    let raw_input = read_to_string("input")?;
    let lines: Vec<&str> = raw_input.split("\n").collect();

    let width: i32 = lines.get(0).unwrap().len().try_into()?;
    let height: i32 = lines.len().try_into()?;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut map: Vec<Vec<char>> = Vec::new();
    // load map and current cursor position
    for (index, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if let Some(cursor) = chars.iter().position(|c| *c == '^') {
            x = cursor.try_into().unwrap();
            y = index.try_into().unwrap();
        }
        map.push(chars);
    }

    Ok((map, width, height, x, y))
}

fn search(mut x: i32, mut y: i32, width: &i32, height: &i32, map: Vec<Vec<char>>) -> Result<HashSet<(i32, i32)>> {
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    let mut direction: Direction = Direction::North; 
    visits.insert((x, y));

    while x > 0 && x < *width && y > 0 && y < *height  {
        // sleep(Duration::from_millis(1));

        let next_location = match direction {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y)
        };

        
        if next_location.0 < 0 || next_location.0 > *width || next_location.1 < 0 || next_location.1 > *height {
            break;
        }
        
        let next_location_value = &map[next_location.1 as usize][next_location.0 as usize];
        
        match next_location_value {
            '#' => {
                // instead of advancing, just turn to the right
                direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North
                }
            },
            _ => {
                // advance the location
                x = next_location.0;
                y = next_location.1;
                visits.insert((x, y));
            }
        }
    }

    Ok(visits)
}