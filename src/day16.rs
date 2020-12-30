use crate::input;
use std::collections::{BTreeSet, HashMap, HashSet};
use regex::{Captures, Regex};

static INPUT: &str = input::_INPUT;

pub fn tickets_part1() -> i32 {
    let allowed_nums: HashSet<i32> = Regex::new(r"(\d+)\-(\d+)").unwrap()
    .captures_iter(
        INPUT.split("\n\n").next().unwrap()
    ).map(|capture| {
        (capture[1].parse().unwrap())..=(capture[2].parse::<i32>().unwrap())
    }).flat_map(|x| x).collect();

    INPUT.split("\n\n").skip(2).next().unwrap()
    .lines().skip(1).flat_map(|line| {
        line.trim().split(',').map(|num| num.trim().parse().unwrap())
    }).filter(|num| !allowed_nums.contains(num)).sum()
}

pub fn tickets_part2() -> i64 {
    let mut rules: HashMap<String, BTreeSet<i32>> = Regex::new(r"([a-zA-Z ]+): (\d+)\-(\d+) or (\d+)\-(\d+)").unwrap()
    .captures_iter(
        INPUT.split("\n\n").next().unwrap()
    ).map(|capture| {
        (capture[1].to_string(), get_full_range(&capture) )
    }).collect();

    let my_ticket: Vec<i32> = INPUT.split("\n\n").skip(1).next().unwrap()
    .lines().skip(1).next().unwrap()
    .split(',').map(|num| num.trim().parse().unwrap()).collect();

    let data: HashSet<Vec<i32>> = INPUT.split("\n\n").skip(2).next().unwrap()
    .lines().skip(1).map(|line| {
        line.split(',').map(|num| num.trim().parse().unwrap()).collect()
    })
    .filter(|ticket: &Vec<i32>| {
        let mut valid = false;
        for value in ticket.iter() {
            valid = false;
            for alloweds in rules.values() {
                valid = valid || alloweds.contains(value);
            }
            if !valid {break;}
            valid = true;
        }
        valid
    })
    .collect();

    let mut new_data: Vec<Vec<i32>> = vec![Vec::new(); my_ticket.len()];

    for ticket in data.into_iter() {
        for (index, value) in ticket.iter().enumerate() {
            new_data[index].push(*value);
        }
    }

    let mut field_indexes: HashMap<&str, usize> = HashMap::new();
    let mut used_columns: Vec<usize> = Vec::new();

    loop {
        for (field, range) in rules.iter() {
            let mut possible_columns = Vec::new();
            for (index, value) in new_data.iter().enumerate() {
                if !field_indexes.contains_key(field.as_str()) 
                && !used_columns.contains(&index)
                && range.is_superset(&value.iter().map(|x| *x).collect()){
                    possible_columns.push(index);
                }
            }
            if possible_columns.len() == 1 {
                let index = possible_columns[0];
                field_indexes.insert(field.as_str(), index);
                used_columns.push(index);
                break;
            }
        }
        if used_columns.len() == rules.len() {
            break;
        }
    }

    println!("{} {:?}",used_columns.len(), field_indexes);
    
    let mut prod: i64 = 1;
    for field in field_indexes.keys() {
        if field.starts_with("departure") {
            prod *= my_ticket[*(field_indexes.get(field).unwrap())] as i64;
        }
    }
    prod
}

fn get_full_range(capture: &Captures) -> BTreeSet<i32> {
    (capture[2].parse().unwrap()..=capture[3].parse().unwrap())
    .chain(capture[4].parse().unwrap()..=capture[5].parse().unwrap())
    .collect()
}