use itertools::Itertools;
use crate::input;

pub fn find_product_sum_2020(combinations: usize) -> i32{
    let sums: Vec<i32> = input::_INPUT.split('\n')
        .map(|x| {
            x.trim()
            .parse::<i32>()
            .unwrap()
        })
        .combinations(combinations)
        .find(|x| {
            x.iter()
            .sum::<i32>() == 2020
        }).unwrap();
    sums.iter().product()
}