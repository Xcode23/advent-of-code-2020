use std::collections::HashSet;

use crate::input;
use Move::*;

static INPUT: &str = input::_INPUT;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Move {
    E,
    W,
    NW,
    NE,
    SW,
    SE
}

pub fn black_tiles_part1() -> i64 {
    let mut buffer = Vec::new();
    let data: Vec<Vec<Move>> = INPUT.lines().map(|line| {
        line.trim().chars().fold(Vec::new(),|mut acc, char_move| {
            match char_move {
                'e' => {
                    if buffer.is_empty() {
                        acc.push(E);
                    } else {
                        match buffer.pop().unwrap() {
                            'n' => acc.push(NE),
                            's' => acc.push(SE),
                            _ => panic!("bullshit direction")
                        }
                    }
                },
                'w' => {
                    if buffer.is_empty() {
                        acc.push(W);
                    } else {
                        match buffer.pop().unwrap() {
                            'n' => acc.push(NW),
                            's' => acc.push(SW),
                            _ => panic!("bullshit direction")
                        }
                    }
                },
                'n' => {buffer.push('n');},
                's' => {buffer.push('s');},
                _   => panic!("bullshit direction")
            }
            acc
        })
    }).collect();
    let mut black_tiles = HashSet::new();
    for tile in data {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for mov in tile {
            match mov {
                E   => {x += 1; y -= 1;},
                W   => {x -= 1; y += 1;},
                NE  => {x += 1; z -= 1;},
                SW  => {x -= 1; z += 1;},
                NW  => {y += 1; z -= 1;},
                SE  => {y -= 1; z += 1;}
            }
        }
        if !black_tiles.insert((x,y,z)) {
            black_tiles.remove(&(x,y,z));
        }
    }
    black_tiles.len() as i64
}

pub fn black_tiles_part2() -> i64 {
    let mut buffer = Vec::new();
    let data: Vec<Vec<Move>> = INPUT.lines().map(|line| {
        line.trim().chars().fold(Vec::new(),|mut acc, char_move| {
            match char_move {
                'e' => {
                    if buffer.is_empty() {
                        acc.push(E);
                    } else {
                        match buffer.pop().unwrap() {
                            'n' => acc.push(NE),
                            's' => acc.push(SE),
                            _ => panic!("bullshit direction")
                        }
                    }
                },
                'w' => {
                    if buffer.is_empty() {
                        acc.push(W);
                    } else {
                        match buffer.pop().unwrap() {
                            'n' => acc.push(NW),
                            's' => acc.push(SW),
                            _ => panic!("bullshit direction")
                        }
                    }
                },
                'n' => {buffer.push('n');},
                's' => {buffer.push('s');},
                _   => panic!("bullshit direction")
            }
            acc
        })
    }).collect();
    let mut black_tiles = HashSet::new();
    for tile in data {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for mov in tile {
            match mov {
                E   => {x += 1; y -= 1;},
                W   => {x -= 1; y += 1;},
                NE  => {x += 1; z -= 1;},
                SW  => {x -= 1; z += 1;},
                NW  => {y += 1; z -= 1;},
                SE  => {y -= 1; z += 1;}
            }
        }
        if !black_tiles.insert((x,y,z)) {
            black_tiles.remove(&(x,y,z));
        }
    }

    for _ in 0..100 {
        let mut to_flip = HashSet::new();
        for tile in black_tiles.iter() {
            let black_neighbours = get_black_neighbours(*tile, &black_tiles);
            if black_neighbours == 0 || black_neighbours > 2 {
                to_flip.insert(*tile);
            }
            let neighbours = get_neighbours(*tile);
            for neighbour in neighbours {
                if !black_tiles.contains(&neighbour) 
                    && get_black_neighbours(neighbour, &black_tiles) == 2 {
                    to_flip.insert(neighbour);
                }
            }
        }
        for tile in to_flip {
            if !black_tiles.insert(tile) {
                black_tiles.remove(&tile);
            }
        }
    }
    black_tiles.len() as i64
}

fn get_neighbours((x,y,z): (i32,i32,i32)) -> Vec<(i32,i32,i32)> {
    vec![(x+1, y-1,z),(x-1, y+1,z),(x+1, y,z-1),(x-1, y,z+1),(x, y+1,z-1),(x, y-1,z+1)]
}

fn get_black_neighbours((x,y,z): (i32,i32,i32), black_tiles: &HashSet<(i32,i32,i32)>) -> i64 {
    let mut result = 0;
    for neighbour in get_neighbours((x,y,z)) {
        if black_tiles.contains(&neighbour) {
            result += 1;
        }
    }
    result
}