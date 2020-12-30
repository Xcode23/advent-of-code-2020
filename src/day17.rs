use crate::input;
use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = input::_INPUT;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct HyperCube {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

struct Step3(i32, i32, i32);
struct Step4(i32, i32, i32, i32);

pub fn cubes_part1() -> i32 {
    let mut previous_cycle: HashSet<Cube> = INPUT.lines().enumerate().flat_map(move |(y, row)| {
        row.chars().enumerate().filter(|(_,column)| {
            *column == '#'
        }).map(move |(x,_)| {
            Cube::new().add(Step3(x as i32,y as i32,0))
        })
    }).collect();

    for _ in 0..6 {
        let mut neighbours = HashSet::new();
        let mut new_cycle = HashSet::new();
        for cube in previous_cycle.iter() {
            let temp_neighbours = cube.neighbours();
            let neighbour_count = previous_cycle.intersection(&temp_neighbours).count();
            neighbours.extend(temp_neighbours);
            if neighbour_count == 2 || neighbour_count == 3 {
                new_cycle.insert(*cube);
            }
        }

        for cube in neighbours.difference(&previous_cycle) {
            let neighbour_count= previous_cycle.intersection(&cube.neighbours()).count();
            if neighbour_count == 3 {
                new_cycle.insert(*cube);
            }
        }
        previous_cycle = new_cycle;
    }
    previous_cycle.len() as i32
}

pub fn cubes_part2() -> i32 {
    let mut previous_cycle: HashSet<HyperCube> = INPUT.lines().enumerate().flat_map(move |(y, row)| {
        row.chars().enumerate().filter(|(_,column)| {
            *column == '#'
        }).map(move |(x,_)| {
            HyperCube::new().add(Step4(x as i32,y as i32,0,0))
        })
    }).collect();

    for _ in 0..6 {
        let mut neighbours = HashSet::new();
        let mut new_cycle = HashSet::new();
        for hyper_cube in previous_cycle.iter() {
            let temp_neighbours = hyper_cube.neighbours();
            let neighbour_count = previous_cycle.intersection(&temp_neighbours).count();
            neighbours.extend(temp_neighbours);
            if neighbour_count == 2 || neighbour_count == 3 {
                new_cycle.insert(*hyper_cube);
            }
        }

        for hyper_cube in neighbours.difference(&previous_cycle) {
            let neighbour_count= previous_cycle.intersection(&hyper_cube.neighbours()).count();
            if neighbour_count == 3 {
                new_cycle.insert(*hyper_cube);
            }
        }
        previous_cycle = new_cycle;
    }
    previous_cycle.len() as i32
}

impl Cube {

    fn neighbours(&self) -> HashSet<Cube> {
        (0..3).map(|_| -1..=1).multi_cartesian_product().filter(|offsets| {
            !offsets.iter().all(|value| *value == 0)
        })
        .map(|offsets| self.add(Step3(offsets[0], offsets[1], offsets[2])))
        .collect()
    }

    fn new() -> Cube {
        Cube{
            x:0,
            y:0,
            z:0
        }
    }

    fn add(self, Step3(x,y,z): Step3) -> Cube {
        let mut new_cube = self;
        new_cube.x += x;
        new_cube.y += y;
        new_cube.z += z;
        new_cube
    }
}

impl HyperCube {

    fn neighbours(&self) -> HashSet<HyperCube> {
        (0..4).map(|_| -1..=1).multi_cartesian_product().filter(|offsets| {
            !offsets.iter().all(|value| *value == 0)
        })
        .map(|offsets| self.add(Step4(offsets[0], offsets[1], offsets[2], offsets[3])))
        .collect()
    }

    fn new() -> HyperCube {
        HyperCube{
            x:0,
            y:0,
            z:0, 
            w:0
        }
    }

    fn add(self, Step4(x,y,z,w): Step4) -> HyperCube {
        let mut new_hyper_cube = self;
        new_hyper_cube.x += x;
        new_hyper_cube.y += y;
        new_hyper_cube.z += z;
        new_hyper_cube.w += w;
        new_hyper_cube
    }
}