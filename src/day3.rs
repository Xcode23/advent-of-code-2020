use crate::input;

struct TreeMap{
    location: (usize, usize),
    map: Vec<Vec<bool>>
}

impl TreeMap{
    fn new(input: &str) -> TreeMap{
        let parsed_input: Vec<Vec<bool>> = input.split('\n').map(|line| {
            line.trim().chars().map(|place| {
                match place {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _   => Err(panic!("bullshit character"))
                }.unwrap()
            }).collect()
        }).collect();
        TreeMap{
            location: (0,0),
            map: parsed_input
        }
    }

    fn move_in_map(&mut self, x:i32, y:i32){
        self.location.0 += x as usize;
        self.location.1 += y as usize;
    }

    fn location(&self) -> (i32, i32){
        (self.location.0 as i32, self.location.1 as i32)
    }

    fn is_tree(&self) -> bool{
        let size = (self.map[0].len(), self.map.len());
        if self.map[self.location.1%size.1][self.location.0%size.0] == true {true} else {false}
    }

    fn map_len(&self) -> i32{
        self.map.len() as i32
    }
}

pub fn number_of_trees(slopes: Vec<(i32,i32)>) -> i64{
    let mut my_map = TreeMap::new(input::_INPUT);
    let size = my_map.map_len();
    let mut counter = 1;
    for slope in slopes{
        let mut slope_counter = 0;
        while my_map.location().1 < size {
            if my_map.is_tree() {slope_counter+=1;}
            my_map.move_in_map(slope.0, slope.1);
        }
        counter *= slope_counter;
        my_map.location = (0,0);
    }
    counter
}