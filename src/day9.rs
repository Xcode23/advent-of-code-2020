use crate::input;
use itertools::Itertools;

pub fn first_weakness() -> i32 {
    let data: Vec<i64> = input::_INPUT.lines().map(|line| line.trim().parse().unwrap()).collect();
    for (index, value) in data.iter().enumerate() {
        if index >= 25 {
            let is_sum_flag = &data[index-25..index].iter()
                .combinations(2).find(|x| x.iter().map(|y| **y).sum::<i64>() == *value);
            if let None = is_sum_flag {
                return *value as i32
            }
        }
    }
    0
}

pub fn weakness() -> i64 {
    let target = first_weakness() as i64;
    let data: Vec<i64> = input::_INPUT.lines().map(|line| line.trim().parse().unwrap()).collect();
    for (index, _) in data.iter().enumerate() {
        for length in 2..=(data.len()-index) {
            let slice = &data[index..index+length];
            let result: i64 = slice.iter().map(|x| *x).sum();
            if result == target {
                return slice.iter().max().unwrap() + slice.iter().min().unwrap();
            }
        }
    }
    0
}