use std::{fs::File, io::Read, fs::read_to_string};

use regex::Regex;

use anyhow::Result;

fn main() -> Result<()> {
    let raw_input = read_to_string("input")?;
    let split_input = raw_input.split("don't()").collect::<Vec<&str>>();
    let mut iter = split_input.into_iter();
    let first_total = process_section(iter.next().unwrap());

    let totals: i32 = iter.map(|item| {
        // we already removed all the don't, so we just need to ignore the first thing before each do()
        match item.split_once("do()") {
            Some((_ignore, do_string)) => process_section(do_string),
            None => 0
        }
    }
    ).collect::<Vec<i32>>().into_iter().sum();
    dbg!(totals + first_total);
    Ok(())
}

fn process_section(raw_input: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();
    let results: i32 = re.captures_iter(&raw_input).map(|capture| {
        let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();
        x * y
    }).sum();
    results
}


// fn main() -> Result<()> {
//     let raw_input = read_to_string("input")?;

//     let re: Regex = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();

//     let results: i32 = re.captures_iter(&raw_input).map(|capture| {
//         let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
//         let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();
//         x * y
//     }).sum();
//     dbg!(results);

//     Ok(())
// }