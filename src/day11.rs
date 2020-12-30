use crate::input;
use itertools::Itertools;
use Seat::{Occupied, Empty, Floor};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Seat {
    Empty,
    Occupied,
    Floor
}

pub fn occupied_seats() -> i32 {
    let mut data: Vec<Vec<Seat>> = input::_INPUT.lines().map(|line| {
        line.trim().chars().map(|seat| {
            match seat {
                '#' => Occupied,
                'L' => Empty,
                _   => Floor
            }
        }).collect()
    }).collect();
    let mut new_data: Vec<Vec<Seat>> = data.clone();
    for (index1, _) in data.iter().enumerate() {
        for (index0, _) in data[index1].iter().enumerate() {
            new_data[index1][index0] = data.new_state((index0, index1));
        }
    }
    while new_data != data {
        data = new_data;
        new_data = data.clone();
        for (index1, _) in data.iter().enumerate() {
            for (index0, _) in data[index1].iter().enumerate() {
                new_data[index1][index0] = data.new_state((index0, index1));
            }
        }
    }
    new_data.iter().map(|line| line.iter().filter(|seat| **seat==Occupied).count()).sum::<usize>() as i32
}

pub fn part2_occupied_seats() -> i32 {
    let mut data: Vec<Vec<Seat>> = input::_INPUT.lines().map(|line| {
        line.trim().chars().map(|seat| {
            match seat {
                '#' => Occupied,
                'L' => Empty,
                _   => Floor
            }
        }).collect()
    }).collect();
    let mut new_data: Vec<Vec<Seat>> = data.clone();
    for (index1, _) in data.iter().enumerate() {
        for (index0, _) in data[index1].iter().enumerate() {
            new_data[index1][index0] = data.second_new_state((index0, index1));
        }
    }
    while new_data != data {
        data = new_data;
        new_data = data.clone();
        for (index1, _) in data.iter().enumerate() {
            for (index0, _) in data[index1].iter().enumerate() {
                new_data[index1][index0] = data.second_new_state((index0, index1));
            }
        }
    }
    new_data.iter().map(|line| line.iter().filter(|seat| **seat==Occupied).count()).sum::<usize>() as i32
}

trait Seats {
    fn new_state(&self, seat: (usize, usize)) -> Seat;
    fn second_new_state(&self, seat: (usize, usize)) -> Seat;
    fn check_direction(&self, seat: (usize, usize), offsets: (i32,i32)) -> bool;
}

impl Seats for Vec<Vec<Seat>> {
    fn new_state(&self, seat: (usize, usize)) -> Seat{
        let mut occupied_surrounding_seats = 0;
        for offsets in (-1..=1).cartesian_product(-1..=1) {
            if offsets == (0,0) {continue;}
            let index0 = seat.0 as i32 + offsets.0;
            if index0 < 0 || index0 as usize >= self[0].len() {continue;}
            let index0 = index0 as usize;
            let index1 = seat.1 as i32 + offsets.1;
            if index1 < 0 || index1 as usize >= self.len() {continue;}
            let index1 = index1 as usize;
            let surrounding_seat = &self[index1][index0];
            if let Occupied = surrounding_seat {
                occupied_surrounding_seats += 1;
            }
        }
        if occupied_surrounding_seats == 0 && self[seat.1][seat.0] == Empty {return Occupied;}
        if occupied_surrounding_seats >= 4 && self[seat.1][seat.0] == Occupied {return Empty;}
        self[seat.1][seat.0]
    }

    fn second_new_state(&self, seat: (usize, usize)) -> Seat{
        let mut occupied_surrounding_seats = 0;

        if self.check_direction(seat, (0,1)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (1,0)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (1,1)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (0,-1)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (-1,0)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (-1,-1)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (1,-1)) {occupied_surrounding_seats += 1;}
        if self.check_direction(seat, (-1,1)) {occupied_surrounding_seats += 1;}
        
        if occupied_surrounding_seats == 0 && self[seat.1][seat.0] == Empty {return Occupied;}
        if occupied_surrounding_seats >= 5 && self[seat.1][seat.0] == Occupied {return Empty;}
        self[seat.1][seat.0]
    }

    fn check_direction(&self, seat: (usize, usize), offsets: (i32,i32)) -> bool{
        let mut new_offsets = offsets;
        loop {
            let index0 = seat.0 as i32 + new_offsets.0;
            if index0 < 0 || index0 as usize >= self[0].len() {return false;}
            let index0 = index0 as usize;
            let index1 = seat.1 as i32 + new_offsets.1;
            if index1 < 0 || index1 as usize >= self.len() {return false;}
            let index1 = index1 as usize;
            if self[index1][index0] == Occupied {return true;}
            if self[index1][index0] == Empty {return false;}
            new_offsets = (new_offsets.0 + offsets.0, new_offsets.1 + offsets.1);
        }
    }
}