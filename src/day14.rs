use crate::input;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use itertools::Itertools;

pub fn memory_sum_part1() -> i64 {
    let mut mask= "";
    let mut memory: HashMap<i64,i64> = HashMap::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    input::_INPUT.lines().for_each(|line| {
        if line.starts_with("mask") {
            mask = line.strip_prefix("mask = ").unwrap().trim();
        } else {
            let mut captured_nums = num_regex.captures_iter(line);
            let address: i64 = captured_nums.next().unwrap()[0].parse().unwrap();
            let mut value: i64 = captured_nums.next().unwrap()[0].parse().unwrap();
            for (index, bit) in mask.chars().rev().enumerate() {
                match bit {
                    '1' => value |= 0x1 << index, 
                    '0' => value &= !(0x1 << index),
                    _   => {},
                }
            }
            memory.insert(address, value);
        }
    });
    memory.values().sum()
}

pub fn memory_sum_part2() -> i64 {
    let mut mask= "";
    let mut memory: HashMap<i64,i64> = HashMap::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    input::_INPUT.lines().for_each(|line| {
        if line.starts_with("mask") {
            mask = line.strip_prefix("mask = ").unwrap().trim();
        } else {
            let mut captured_nums = num_regex.captures_iter(line);
            let mut address: i64 = captured_nums.next().unwrap()[0].parse().unwrap();
            let value: i64 = captured_nums.next().unwrap()[0].parse().unwrap();
            let mut true_addresses = HashSet::new();
            let mut bits_to_flip = Vec::new();
            for (index, bit) in mask.chars().rev().enumerate() {
                match bit {
                    '1' => address |= 0x1 << index, 
                    '0' => {},
                    _   => {bits_to_flip.push(index)},
                }
            }
            
            for len in 0..=bits_to_flip.len() {
                for comb in bits_to_flip.iter().combinations(len) {
                    let mut temp_address = address;
                    for bit in bits_to_flip.iter() {
                        if comb.contains(&&bit){
                            temp_address |= 0x1 << bit;
                        } else {
                            temp_address &= !(0x1 << bit);
                        }
                    }
                    true_addresses.insert(temp_address);
                }
            }
            
            for address in true_addresses.iter() {
                memory.insert(*address, value);
            }
        }
    });
    memory.values().sum()
}