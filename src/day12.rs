use crate::input;
use Direction::{North, West, South, East};
use Move::{Cardinal, Left, Right, Forward};

#[derive(Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East
}

#[derive(Clone, Copy)]
enum Move {
    Cardinal(Direction, i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

struct Position<T> {
    ns: i32,
    we: i32,
    orientation: T
}

#[derive(Clone, Copy)]
struct Waypoint {
    ns: i32,
    we: i32
}

trait Orientation {
    fn starting_orientation() -> Self;
    fn new_orientation(&self, new_move: Move) -> Self;
    fn new_change(&self, new_move: Move) -> (i32,i32);
}

pub fn distance_part1() -> i32 {
    input::_PEDRO_INPUT.lines()
        .map(|line| new_move(line.trim()))
        .fold(Position::<Direction>::new(), |pos, mov| pos.new_position(mov))
        .distance()
}

pub fn distance_part2() -> i32 {
    input::_PEDRO_INPUT.lines()
        .map(|line| new_move(line.trim()))
        .fold(Position::<Waypoint>::new(), |pos, mov| pos.new_position(mov))
        .distance()
}

fn new_move(str_move: &str) -> Move {
    let move_type = str_move.chars().next().unwrap();
    let distance: i32 = str_move[1..].parse().unwrap();
    match move_type {
        'F' => Forward(distance),
        'R' => Right(distance),
        'L' => Left(distance),
        'N' => Cardinal(North, distance),
        'S' => Cardinal(South, distance),
        'W' => Cardinal(West, distance),
        _   => Cardinal(East, distance)
    }
}

impl<T: Orientation> Position<T> {
    fn distance(&self) -> i32 {
        self.ns.abs() + self.we.abs()
    }

    fn new() -> Position<T> {
        Position {
            ns: 0,
            we: 0,
            orientation: T::starting_orientation()
        }
    }

    fn new_position(self, new_move: Move) -> Position<T> {
        let (ns, we) = self.orientation.new_change(new_move);
        let new_ori = self.orientation.new_orientation(new_move);
        Position{
            ns: ns + self.ns,
            we: we + self.we,
            orientation: new_ori
        }
    }
}

impl Waypoint {
    fn rotate_waypoint(self) -> Waypoint {
        Waypoint {
            ns: -self.we,
            we: self.ns
        }
    }
    
    fn counter_rotate_waypoint(self) -> Waypoint {
        Waypoint {
            ns: self.we,
            we: -self.ns
        }
    }
}

impl Direction {

    fn add(self, degrees: i32) -> Direction {
        let mut result = self;
        for _ in 0..(degrees / 90) {
            result = result.succ();
        }
        result
    }

    fn sub(self, degrees: i32) -> Direction {
        let mut result = self;
        for _ in 0..(degrees / 90) {
            result = result.prev();
        }
        result
    }

    fn succ(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }

    fn prev(self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South
        }
    }
}

impl Orientation for Direction {
    fn starting_orientation() -> Self {
        East
    }
    fn new_orientation(&self, new_move: Move) -> Self {
        match new_move {
            Right(degrees)  => self.add(degrees),
            Left(degrees)   => self.sub(degrees),
            _                   => *self
        }
    }
    fn new_change(&self, new_move: Move) -> (i32,i32) {
        let mut ns = 0;
        let mut we = 0;
        match new_move {
            Cardinal(North, distance) => ns += distance,
            Cardinal(South, distance) => ns -= distance,
            Cardinal(West, distance) => we -= distance,
            Cardinal(East, distance) => we += distance,
            Forward(distance) => match self {
                North => ns += distance,
                South => ns -= distance,
                West  => we -= distance,
                East  => we += distance
            }
            _ => {}
        }
        (ns, we)
    }
}

impl Orientation for Waypoint {
    fn starting_orientation() -> Self {
        Waypoint {
            ns: 1,
            we: 10
        }
    }

    fn new_orientation(&self, new_move: Move) -> Self {
        let mut waypoint = *self;
        match new_move {
            Cardinal(North, distance) => waypoint.ns += distance,
            Cardinal(South, distance) => waypoint.ns -= distance,
            Cardinal(West, distance) => waypoint.we -= distance,
            Cardinal(East, distance) => waypoint.we += distance,
            Right(degrees) =>  {
                for _ in 0..(degrees/90) {
                    waypoint = waypoint.rotate_waypoint();
                }
            },
            Left(degrees) => {
                for _ in 0..(degrees/90) {
                    waypoint = waypoint.counter_rotate_waypoint();
                }
            },
            _ => {}
        }
        waypoint
    }

    fn new_change(&self, new_move: Move) -> (i32,i32) {
        let mut ns = 0;
        let mut we = 0;
        match new_move {
            Forward(distance) => {
                ns += self.ns * distance;
                we += self.we * distance;
            }
            _ => {}
        }
        (ns, we)
    }
}