use std::{fs::File, io::Read, fs::read_to_string};

use regex::Regex;

use anyhow::Result;

fn main() -> Result<()> {
    let raw_input = read_to_string("input")?;

    Regex::new(r'(?P<test>\d+?): (.+?)\n')?.captures_iter().map(|row| {
        dbg!(row);
    });

    // let split_input = raw_input.split("\r\n").collect::<Vec<&str>>();



    // dbg!(split_input.len());
    Ok(())
    
}
