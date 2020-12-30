use std::collections::HashMap;

use crate::input;

static INPUT: &str = input::_INPUT;

pub fn crab_cups_part1() -> i64 {
    let mut data = HashMap::new();
    let starting_value;
    {
        let cups: Vec<i64> = INPUT.chars()
            .map(|cup| cup.to_digit(10).unwrap() as i64).collect();
        starting_value = cups[0];
        for index in 0..(cups.len()-1) {
            data.insert(cups[index], cups[index + 1]);
        }
        data.insert(cups[cups.len() - 1], starting_value);
    }
    play(&mut data, starting_value, 100);
    let mut print_aux = data[&1];
    while print_aux != 1 {
        print!("{}", print_aux);
        print_aux = data[&print_aux];
    }
    println!("");
    0
}

pub fn crab_cups_part2() -> i64 {
    let mut data = HashMap::new();
    let starting_value;
    {
        let cups: Vec<i64> = INPUT.chars()
            .map(|cup| cup.to_digit(10).unwrap() as i64).collect();
        starting_value = cups[0];
        let max_value = *cups.iter().max().unwrap();
        for index in 0..(cups.len()-1) {
            data.insert(cups[index], cups[index + 1]);
        }
        data.insert(cups[cups.len() - 1], max_value + 1);
        for cup in (max_value + 1)..1_000_000 {
            data.insert(cup, cup + 1);
        }
        data.insert(1_000_000, starting_value);
    }
    play(&mut data, starting_value, 10_000_000);
    data[&1] * data[&data[&1]]
}

fn play(data: &mut HashMap<i64, i64>, starting_point: i64, rounds: i64) {
    let mut current_cup = starting_point;
    let max_cup = *data.keys().max().unwrap();
    let min_cup = *data.keys().min().unwrap();
    for _ in 0..rounds {
        let cup_to_move1 = data[&current_cup];
        let cup_to_move2 = data[&cup_to_move1];
        let cup_to_move3 = data[&cup_to_move2];
        let mut destination_cup =  current_cup;
        loop {
            if destination_cup == min_cup {
                destination_cup = max_cup + 1;
            }
            destination_cup -= 1;
            if  destination_cup != cup_to_move1 && 
                destination_cup != cup_to_move2 && 
                destination_cup != cup_to_move3 {
                    break;
                }
        }
        *data.get_mut(&current_cup).unwrap() = data[&cup_to_move3];
        *data.get_mut(&cup_to_move3).unwrap() = data[&destination_cup];
        *data.get_mut(&destination_cup).unwrap() = cup_to_move1;
        current_cup = data[&current_cup];
    }
}