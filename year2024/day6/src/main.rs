use std::{fs::read_to_string};
use anyhow::Result;
use std::collections::HashSet;

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}


// fn render_map(map: &Vec<Vec<char>>, visited: &HashSet<(i32, i32)>, obstacle: Option<(&i32, &i32)>) {
//     println!("NEW");
//     for (y, row) in map.iter().enumerate() {
//         let new_row: String = row.iter().enumerate().map(|(x, c)| {
//             if let Some((obstacle_x, obstacle_y)) = obstacle {
//                 if x as i32 == *obstacle_x && y as i32 == *obstacle_y  {
//                     // render the obstacle here!
//                     return '@';
//                 }
//             }

//             if visited.contains(&(x as i32, y as i32)) {
//                 return 'X';
//             }

//             return c.clone();
//         }).into_iter().collect();

//         println!("{y:03}{}", new_row);
//     }
// }


fn main() -> Result<()> {
    let (map, width, height, x, y) = load_map().unwrap();
    let (_, visits) = search(x.clone(), y.clone(), &width, &height, &map, None)?;

    dbg!("fell out of walking with ? visits", visits.len());

    let loop_count: i32 = visits.par_iter().map(|visited_location| {
        let search_x = &visited_location.0;
        let search_y = &visited_location.1;
        let (looped, _new_visited) = search(x, y, &width, &height, &map, Some((search_x, search_y))).unwrap();
        if looped {
            1
        } else {
            0
        }
    }).sum();

    dbg!("Found {loop_count:?} loop locations!", {loop_count});

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

fn search(mut x: i32, mut y: i32, width: &i32, height: &i32, map: &Vec<Vec<char>>, obstacle: Option<(&i32, &i32)>) -> Result<(bool, HashSet<(i32, i32)>)> {
    let mut visits: HashSet<(i32, i32)> = HashSet::new(); // this tracks every location we visited
    let mut directional_visits: HashSet<(i32, i32, Direction)> = HashSet::new(); // this tracks all spots and directions, and shouldn't repeat without a loop

    let mut direction: Direction = Direction::North; 
    visits.insert((x, y));
    directional_visits.insert((x, y, direction));
    

    let mut obstacle_x = -200;
    let mut obstacle_y = -200;

    if let Some((&tmp_x, &tmp_y)) = obstacle {
        obstacle_x = tmp_x;
        obstacle_y = tmp_y;
    }

    while x > 0 && x < *width && y > 0 && y < *height  {
        let next_location = match direction {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y)
        };
        
        if next_location.0 < 0 || next_location.0 > *width - 1 || next_location.1 < 0 || next_location.1 > *height -1 {
            break;
        }
        
        let next_location_value = if next_location.0 == obstacle_x && next_location.1 == obstacle_y {
            // this is the searched location. We pretend to find a fake obstacle
            // note: use the next_location not the current one, or this is going to be weird!
            &'#'
        } else {
            &map[next_location.1 as usize][next_location.0 as usize]
        };
         
        
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

        if directional_visits.contains(&(x, y, direction)) {
            return Ok((true, visits));
        }
        directional_visits.insert((x, y, direction));
    }

    Ok((false, visits))
}