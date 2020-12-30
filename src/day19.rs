use crate::input;
use std::collections::HashMap;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use bincode::{serialize, deserialize};

static INPUT: &str = input::_INPUT;

type Rule = Vec<i64>;

pub fn valid_messages_part1() -> i64 {
    let mut allowed_values: HashMap<i64,Vec<String>> = HashMap::new();
    let mut incomplete_rules: HashMap<i64,Vec<Rule>> = HashMap::new();
    let rules = INPUT.split("\n\n").next().unwrap();
    for rule in rules.lines() {
        let mut parts = rule.split(':').map(|part| part.trim());
        let index: i64 = parts.next().unwrap().parse().unwrap();
        let rule_content = parts.next().unwrap();
        if rule_content.starts_with("\"") {
            allowed_values.insert(index, vec![(&rule_content[1..rule_content.len()-1]).to_string()]);
        } else {
            let new_rules = rule_content.split("|").map(|new_rule| {
                new_rule.trim().split(" ").map(|rule_index| rule_index.trim().parse().unwrap()).collect()
            }).collect();
            incomplete_rules.insert(index, new_rules);
        }

    }
    let allowed_messages: &Vec<String> = values_for_index(0, &mut allowed_values, &mut incomplete_rules);
    let messages: Vec<&str> = INPUT.split("\n\n").skip(1).next().unwrap().lines().collect();
    messages.iter().filter(|&&message| allowed_messages.contains(&message.to_string())).count() as i64
}

fn values_for_index<'a>(index: i64, allowed_values: &'a mut HashMap<i64,Vec<String>>, incomplete_rules: &mut HashMap<i64,Vec<Rule>>) -> &'a Vec<String> {
    if !incomplete_rules.contains_key(&index) && allowed_values.contains_key(&index) {
        return allowed_values.get(&index).unwrap();
    }

    let mut new_values = Vec::new();
    let rules_set = incomplete_rules.remove(&index).unwrap();
    for rules in rules_set {
        let mut values_from_rule = Vec::new();
        for rule_index in rules.iter() {
            let substrings_from_index = values_for_index(*rule_index, allowed_values, incomplete_rules);
            values_from_rule.push(substrings_from_index.clone());
        }
        let values_from_rule: Vec<String> = values_from_rule.into_iter()
        .multi_cartesian_product().map(|substrings| {
            substrings.iter().map(|substring| substring).join("")
        }).collect();
        new_values.extend(values_from_rule.into_iter());
    }
    allowed_values.insert(index, new_values);

    allowed_values.get(&index).unwrap()
}

pub fn valid_messages_part2() -> i64 {
    let mut allowed_values: HashMap<i64,Vec<String>> = HashMap::new();
    let mut incomplete_rules: HashMap<i64,Vec<Rule>> = HashMap::new();
    let rules = INPUT.split("\n\n").next().unwrap();
    for rule in rules.lines() {
        let mut parts = rule.split(':').map(|part| part.trim());
        let index: i64 = parts.next().unwrap().parse().unwrap();
        let rule_content = parts.next().unwrap();
        if rule_content.starts_with("\"") {
            allowed_values.insert(index, vec![(&rule_content[1..rule_content.len()-1]).to_string()]);
        } else {
            let new_rules = rule_content.split("|").map(|new_rule| {
                new_rule.trim().split(" ").map(|rule_index| rule_index.trim().parse().unwrap()).collect()
            }).collect();
            incomplete_rules.insert(index, new_rules);
        }

    }
    //generate_values(&mut allowed_values, &mut incomplete_rules);
    //get_saved_values(&mut allowed_values, &mut incomplete_rules);

    values_for_index(42, &mut allowed_values, &mut incomplete_rules);
    values_for_index(31, &mut allowed_values, &mut incomplete_rules);

    let substrings_42 = allowed_values.get(&42).unwrap();
    let substrings_31 = allowed_values.get(&31).unwrap();

    let messages = INPUT.split("\n\n").skip(1).next().unwrap().lines();
    let valid_messages: Vec<&str> = messages.filter(|message| {
        let mut remainder = *message;
        let mut counter_42 = 0;
        let mut counter_31 = 0;
        let mut flag_31 = false;
        if let Some(prefix) = substrings_42.iter().find(|substring| remainder.starts_with(substring.as_str())) {
            remainder = remainder.strip_prefix(prefix).unwrap();
            counter_42 += 1;
        } else {
            return false;
        }
        if let Some(prefix) = substrings_42.iter().find(|substring| remainder.starts_with(substring.as_str())) {
            remainder = remainder.strip_prefix(prefix).unwrap();
            counter_42 += 1;
        } else {
            return false;
        }
        
        while !remainder.is_empty() {
            let changes = remainder;
            if !flag_31 {
                if let Some(prefix) = substrings_42.iter().find(|substring| remainder.starts_with(substring.as_str())){
                    remainder = remainder.strip_prefix(prefix).unwrap();
                    counter_42 += 1;
                }
            }
            if let Some(prefix) = substrings_31.iter().find(|substring| remainder.starts_with(substring.as_str())){
                remainder = remainder.strip_prefix(prefix).unwrap();
                counter_31 += 1;
                flag_31 = true;
            }

            if changes == remainder {
                return false;
            }
        }
        if counter_31 + 1  > counter_42 || counter_31 < 1 {
            return false;
        } 
        true
    }).collect();
    valid_messages.len() as i64
}

fn generate_values(allowed_values: &mut HashMap<i64,Vec<String>>, incomplete_rules: &mut HashMap<i64,Vec<Rule>>) {
    let mut file = File::create("42.test").unwrap();
    let substrings_42 = values_for_index(42, allowed_values, incomplete_rules);
    let mut serialization_42 = serialize(substrings_42).unwrap();
    file.write_all(&serialization_42);
    let mut file = File::create("31.test").unwrap();
    let substrings_31 = values_for_index(31, allowed_values, incomplete_rules);
    let mut serialization_31 = serialize(substrings_31).unwrap();
    file.write_all(&serialization_31);
}

fn get_saved_values(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    let deserialized_buffer: Vec<String> = deserialize(&buffer).unwrap();
    deserialized_buffer
}

