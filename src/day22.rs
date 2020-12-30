use std::collections::{HashSet, VecDeque};
use Winner::*;

use crate::input;

static INPUT: &str = input::_INPUT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Winner {
    Player1,
    Player2
}

pub fn combat_part1() -> i64 {
    let mut data = INPUT.split("\n\n").map(|deck| {
        deck.trim().lines().skip(1).map(|card| {
            card.trim().parse().unwrap()
        }).collect::<VecDeque<i64>>()
    });
    let mut deck1 = data.next().unwrap();
    let mut deck2 = data.next().unwrap();
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let mut winner: VecDeque<i64>;
    if deck1.is_empty() {
        winner = deck2;
    } else {
        winner = deck1;
    }

    let mut result = 0;
    let mut counter = 1;
    while !winner.is_empty() {
        result += winner.pop_back().unwrap() * counter;
        counter += 1;
    }
    result
}

pub fn combat_part2() -> i64 {
    let mut data = INPUT.split("\n\n").map(|deck| {
        deck.trim().lines().skip(1).map(|card| {
            card.trim().parse().unwrap()
        }).collect::<VecDeque<i64>>()
    });
    let mut deck1 = data.next().unwrap();
    let mut deck2 = data.next().unwrap();
    let mut winner: VecDeque<i64>;
    match recursive_combat(&mut deck1, &mut deck2) {
        Player1 => winner = deck1,
        Player2 => winner = deck2
    }

    let mut result = 0;
    let mut counter = 1;
    while !winner.is_empty() {
        result += winner.pop_back().unwrap() * counter;
        counter += 1;
    }
    result
}

fn recursive_combat(deck1: &mut VecDeque<i64>, deck2: &mut VecDeque<i64>) -> Winner {
    let mut previous_decks1: HashSet<VecDeque<i64>> = HashSet::new();
    let mut previous_decks2: HashSet<VecDeque<i64>> = HashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        if !previous_decks1.insert(deck1.clone()) || !previous_decks2.insert(deck2.clone()) {
            return Player1;
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if deck1.len() as i64 >= card1 && deck2.len() as i64 >= card2 {
            let mut new_deck1 = VecDeque::from(
                deck1.iter().cloned().collect::<Vec<i64>>()[0..(card1 as usize)].to_vec()
            );
            let mut new_deck2 = VecDeque::from(
                deck2.iter().cloned().collect::<Vec<i64>>()[0..(card2 as usize)].to_vec()
            );
            match recursive_combat(&mut new_deck1, &mut new_deck2) {
                Player1 => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                Player2 => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        } else {
            if card1 > card2 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }

    if deck2.is_empty() {
        Player1
    } else {
        Player2
    }
}