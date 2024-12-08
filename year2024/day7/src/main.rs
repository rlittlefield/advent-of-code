use std::{fs::File, io::Read, fs::read_to_string};

use regex::Regex;
use regex::RegexBuilder;
use itertools::Itertools;
use rayon::prelude::*;
use std::thread::*;
use std::time::Duration;

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Operator {
    Plus,
    Times,
    Merge
}

fn main() -> Result<()> {
    let raw_input = read_to_string("input")?;
    let re = RegexBuilder::new(r"^(?P<test>\d+?)\:(?P<values>.+?)$").multi_line(true).build()?;
    let all_ops = generate_operators_permutations(13);

    let foo: i64 = re.captures_iter(&raw_input).par_bridge().filter_map(|row| {
        let test_value: i64 = row.name("test").expect("no test value").as_str().parse().expect("test value not an int");
        let input_values: Vec<i64> = row.name("values").expect("no input values").as_str().trim().split(" ").filter_map(|num| {
            num.parse().ok()
        }).collect_vec();

        let found_match = (0..65535u32).any(|testing_operators| {
            // let mut new_testing_operators = make_operators(testing_operators);
            // sleep(Duration::from_millis(1000));
            let mut new_testing_operators = all_ops.get(testing_operators as usize).unwrap().iter().rev();

            // dbg!(&new_testing_operators);

            let tested_total = &input_values.iter().fold(0, |acc, i| {
                let op = new_testing_operators.next();

                // dbg!(acc, i, op);
                match op.expect("not enough operators") {
                    Operator::Plus => acc + i,
                    Operator::Times => acc * i,
                    Operator::Merge => {
                        let mut merge_string = acc.to_string();
                        merge_string.push_str(&i.to_string());
                        let result = merge_string.parse().unwrap();
                        // dbg!(&result);
                        result
                    }
                }
            });
            let found_match = tested_total == &test_value;
            if found_match {
                let mut used_ops = all_ops.get(testing_operators as usize).unwrap().iter().rev();
                used_ops.next();

                let mut printable_ops = used_ops.map(|i| {
                    match i {
                        Operator::Plus => String::from("+"),
                        Operator::Times => String::from("*"),
                        Operator::Merge => String::from("||")
                    }
                }).collect_vec();
                printable_ops.truncate(input_values.len());

                let zipped: Vec<String> = input_values
                    .iter()
                    .map(|i| i.to_string())
                    .into_iter()
                    .zip(printable_ops)
                    .flat_map(|(x, y)| vec![x, y])
                    .collect();

                dbg!("Found {:?} with ops", test_value);

                dbg!(zipped);
            }
            found_match
        });

        if found_match {
            Some(test_value)
        } else {
            None
        }
    }).sum();


    dbg!(&foo);
    Ok(())
    
}



fn generate_operators_permutations(max_operators: usize) -> Vec<Vec<Operator>> {
    let variants = vec![Operator::Plus, Operator::Merge, Operator::Times];
    let mut combinations = Vec::new();

    fn generate_recursive(current: &mut Vec<Operator>, max_operators: usize, variants: &[Operator], combinations: &mut Vec<Vec<Operator>>) {
        if current.len() == max_operators {
            combinations.push(current.clone());
            return;
        }

        for &variant in variants {
            current.push(variant);
            generate_recursive(current, max_operators, variants, combinations);
            current.pop();
        }
    }

    generate_recursive(&mut Vec::new(), max_operators, &variants, &mut combinations);
    combinations.into_iter().map(|mut c| {
        c.push(Operator::Plus); // the we need a leading Plus because the reduce starts with 0
        c
    }).collect()
}