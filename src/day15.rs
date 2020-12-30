use crate::input;
use std::collections::HashMap;

pub fn memory_game_part1() -> i32 {
    let data = input::_INPUT.split(',').map(|num| num.trim().parse().unwrap()).collect::<Vec<i32>>();
    let mut round = 1;
    let mut last_rounds: HashMap<i32, i32> = HashMap::new();
    for &starting_num in data.iter() {
        last_rounds.insert(starting_num, round);
        round += 1;
    }
    let mut last_num = *data.last().unwrap();
    last_rounds.remove(&last_num);
    while round <= 2020 {
        let temp;
        if !last_rounds.contains_key(&last_num) {
            temp = 0;
        } else {
            temp = round - 1 - last_rounds.get(&last_num).unwrap();
        }
        last_rounds.insert(last_num, round-1);
        last_num = temp;
        round += 1;
    }
    last_num
}

pub fn memory_game_part2() -> i32 {
    let data = input::_INPUT.split(',').map(|num| num.trim().parse().unwrap()).collect::<Vec<i32>>();
    let mut round = 1;
    let mut last_rounds: HashMap<i32, i32> = HashMap::new();
    for &starting_num in data.iter() {
        last_rounds.insert(starting_num, round);
        round += 1;
    }
    let mut last_num = *data.last().unwrap();
    last_rounds.remove(&last_num);
    while round <= 30000000 {
        let temp;
        if !last_rounds.contains_key(&last_num) {
            temp = 0;
        } else {
            temp = round - 1 - last_rounds.get(&last_num).unwrap();
        }
        last_rounds.insert(last_num, round-1);
        last_num = temp;
        round += 1;
    }
    last_num
}