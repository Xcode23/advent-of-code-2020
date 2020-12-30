use crate::input;

pub fn highest_boardpass_id() -> i32 {
    let seats = get_seats(input::_INPUT);
    seats.into_iter().map(|seat| seat_id(seat)).max().unwrap()
}

pub fn my_seat_id() -> i32 {
    let seats = get_seats(input::_INPUT);
    let mut matrix_seats = [[false; 8]; 128];
    for seat in seats {
        matrix_seats[seat.0 as usize][seat.1 as usize] = true;
    }

    for (index, _) in matrix_seats.iter().enumerate() {
        for (in_index, _) in matrix_seats[index].iter().enumerate() {
            if index > 0 && index < 127 && in_index > 0 && in_index < 7 
                && !matrix_seats[index][in_index]
                && matrix_seats[index][in_index - 1] 
                && matrix_seats[index][in_index + 1] {
                    return seat_id((index as i32, in_index as i32))
                }
        }
    }
    0
}

fn from_pass_to_seat(pass: &str) -> (i32, i32) {
    let mut limits = ((0,127),(0,7));
    for letter in pass.chars() {
        match letter {
            'F' => limits.0.1 = limits.0.0 + (limits.0.1 - limits.0.0) / 2,
            'B' => limits.0.0 = limits.0.0 + (limits.0.1 - limits.0.0) / 2 + 1,
            'L' => limits.1.1 = limits.1.0 + (limits.1.1 - limits.1.0) / 2,
            'R' => limits.1.0 = limits.1.0 + (limits.1.1 - limits.1.0) / 2 + 1,
            _   => panic!("bullshit character\n")
        }
    }
    (limits.0.0,limits.1.0)
}

fn get_seats(input: &str) -> Vec<(i32, i32)> {
    let boardpasses = input::_INPUT.split('\n').map(|x| x.trim());
    let seats = boardpasses.map(|boardpass| from_pass_to_seat(boardpass));
    seats.collect()
}

fn seat_id((seat_x,seat_y): (i32, i32)) -> i32 {
    seat_x * 8 + seat_y
}