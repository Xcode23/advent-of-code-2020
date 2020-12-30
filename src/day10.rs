use crate::input;
use core::panic;
use std::collections::HashMap;

pub fn joltage_all_adapters() -> i32 {
    let mut one_jolts = 0;
    let mut three_jolts = 0;
    let mut data = input::_INPUT.lines().map(|line| 
        line.trim().parse::<i32>().unwrap()
    ).collect::<Vec<_>>();
    data.push(0);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);
    for index in 0..data.len()-1 {
        let adapter = data[index];
        let next = data[index + 1];
        match next - adapter {
            1 => one_jolts += 1,
            2 => {},
            3 => three_jolts += 1,
            _ => panic!("impossible chain: {} -> {}", adapter, next)
        }
    }
    one_jolts * three_jolts
}

pub fn possible_configurations() -> i64 {
    let mut data = input::_INPUT.lines().map(|line| 
        line.trim().parse::<i32>().unwrap()
    ).collect::<Vec<_>>();
    data.push(0);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);
    let mut memory_map = HashMap::new();
    small_configurations(&data, 0, &mut memory_map)
}

fn small_configurations(data: &Vec<i32>, index: usize, subconfigs: &mut HashMap<usize,i64>) -> i64{
    let length = data.len();
    let mut result = 0;

    if index + 1 < length {
        let length_one_configs = if subconfigs.contains_key(&index) {
            *subconfigs.get(&(index+1)).unwrap()
        }else{
            small_configurations(&data, index + 1 , subconfigs)
        };
        if data[index + 1] - data[index] <= 3 { result = length_one_configs; }
        if index + 2 < length {
            let length_two_configs = if subconfigs.contains_key(&index) {
                *subconfigs.get(&(index+2)).unwrap()
            }else{
                small_configurations(&data, index + 2 , subconfigs)
            };
            if data[index + 2] - data[index] <= 3 { result += length_two_configs; }
            if index + 3 < length {
                let length_three_configs = if subconfigs.contains_key(&index) {
                    *subconfigs.get(&(index+3)).unwrap()
                }else{
                    small_configurations(&data, index + 3 , subconfigs)
                };
                if data[index + 3] - data[index] <= 3 { result += length_three_configs; }
            }
        }
    }else{
        result = 1;
    }
    if !subconfigs.contains_key(&index) { subconfigs.insert(index, result); }
    result
}











