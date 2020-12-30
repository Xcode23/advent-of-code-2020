use crate::input;

static INPUT: &str = input::_INPUT;

pub fn encryption_key_part1() -> i64 {
    let card_key: i64;
    let door_key: i64;
    {
        let data: Vec<i64> = INPUT.lines().map(|line| line.trim().parse().unwrap()).collect();
        card_key = data[0];
        door_key = data[1];
    }

    let mut public_key = 1;
    let mut loop_size = 0;
    while public_key != card_key {
        public_key *= 7;
        public_key %= 20201227;
        loop_size += 1;
    }

    let mut encryption_key = 1;
    for _ in 0..loop_size {
        encryption_key *= door_key;
        encryption_key %= 20201227;
    }
    encryption_key
}