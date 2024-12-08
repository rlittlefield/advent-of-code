use std::{fs::File, io::Read, fs::read_to_string};

use regex::Regex;
use regex::RegexBuilder;
use itertools::Itertools;
use rayon::prelude::*;


use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operator {
    Plus,
    Times
}

fn main() -> Result<()> {
    let raw_input = read_to_string("input")?;
    let re = RegexBuilder::new(r"^(?P<test>\d+?)\:(?P<values>.+?)$").multi_line(true).build()?;
    let foo: i64 = re.captures_iter(&raw_input).par_bridge().filter_map(|row| {
        let test_value: i64 = row.name("test").expect("no test value").as_str().parse().expect("test value not an int");
        let input_values: Vec<i64> = row.name("values").expect("no input values").as_str().trim().split(" ").filter_map(|num| {
            num.parse().ok()
        }).collect_vec();

        // every operator needs to exist in loaded_operators once per each slot
        // but that makes too many duplicates
        // let mut loaded_operators: Vec<Operator> = Vec::new();
        // for op in &operators {
        //     for _i in 0..operator_slot_count {
        //         loaded_operators.push(op.clone());
        //     }
        // }

        // unique might be expensive here because it iterates through every combo. Would be better to just not make uniques...
        let found_match = (0..65535u32).any(|testing_operators| {
            let mut new_testing_operators = make_operators(testing_operators);
            new_testing_operators.push(Operator::Plus);

            let tested_total = &input_values.iter().fold(0, |acc, i| {
                match new_testing_operators.pop().expect("not enough operators") {
                    Operator::Plus => acc + i,
                    Operator::Times => acc * i
                }
            });
            let found_match = tested_total == &test_value;
            if found_match {
                let mut used_ops = make_operators(testing_operators);
                used_ops.reverse();

                let mut printable_ops = used_ops.into_iter().map(|i| {
                    match i {
                        Operator::Plus => String::from("+"),
                        Operator::Times => String::from("*")
                    }
                }).collect_vec();
                printable_ops.truncate(input_values.len());

                let zipped: Vec<String> = input_values.iter().map(|i| i.to_string()).into_iter().zip(printable_ops).flat_map(|(x, y)| vec![x, y]).collect();
                println!("Found {:?} with ops", test_value);

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

fn make_operators(n: u32) -> Vec<Operator> {
    let mut ops = Vec::new();
    for i in 0..16 {
        let bit = (n >> i) & 1;
        match bit {
            0 => ops.push(Operator::Plus),
            1 => ops.push(Operator::Times),
            _ => unreachable!()
        }
    }
    ops.reverse();
    // dbg!(&ops);
    ops
}

#[test]
fn test_make_operators() {
    assert_eq!(make_operators(0).pop().unwrap(), Operator::Plus);
    assert_eq!(make_operators(1).pop().unwrap(), Operator::Times);

    let mut two = make_operators(2);
    dbg!(&two);
    assert_eq!(two.pop().unwrap(), Operator::Plus);
    assert_eq!(two.pop().unwrap(), Operator::Times);

    let mut three = make_operators(3);
    dbg!(&three);
    assert_eq!(three.pop().unwrap(), Operator::Times);
    assert_eq!(three.pop().unwrap(), Operator::Times);
    assert_eq!(three.pop().unwrap(), Operator::Plus);
}