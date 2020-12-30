use crate::input;
use std::collections::HashMap;

type BagData = HashMap<String, HashMap<String, i32>>;

pub fn outer_bags() -> i32 {
    let data = parse_input(input::_INPUT);
    data.iter().filter(|(x,_)| data.leads_to_gold((**x).as_str())).collect::<Vec<_>>().len() as i32
}

pub fn inner_bags() -> i32 {
    let data = parse_input(input::_INPUT);
    data.number_of_bags("shiny gold") - 1
}



trait BagDataBehaviour {
    fn contains_gold(&self, bag: &str) -> bool;
    fn leads_to_gold(&self, bag: &str) -> bool;
    fn number_of_bags(&self, bag: &str) -> i32;
}

impl BagDataBehaviour for BagData {

    fn number_of_bags(&self, bag: &str) -> i32{
        if !self.contains_key(bag) {return 1}
        if self.get(bag).unwrap().is_empty() {return 1}
        let mut counter = 1;
        for (new_bag, number) in self.get(bag).unwrap() {
            for num in 0..*number {
                counter += self.number_of_bags(new_bag);
            }
        }
        counter
    }

    fn contains_gold(&self, bag: &str) -> bool {
        if self.contains_key(bag) && self.get(bag).unwrap().contains_key("shiny gold") {
            true
        } else {
            false
        }
    }

    fn leads_to_gold(&self, bag: &str) -> bool {
        if self.contains_gold(bag) {return true;}
        if self.contains_key(bag) {
            let bags = self.get(bag).unwrap();
            for inner_bag in bags.keys() {
                if self.leads_to_gold(inner_bag) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}

fn parse_contents(contents: &str) -> HashMap<String, i32> {
    let mut result = HashMap::new();
    if &contents[0..2] != "no" {
        contents.split(',').map(|content| {
            content.trim().split(' ').collect::<Vec<_>>()
        }).for_each(|content| {
            result.insert(format!("{} {}",content[1],content[2]), content[0].parse().unwrap());
        });
    }
    result
}

fn parse_line(line: &str) -> (String, HashMap<String, i32>) {
    let parsed_line: Vec<&str> = line.trim()
        .split("contain")
        .map(|part| part.trim()).collect();
    let keys = parsed_line[0].split(' ').map(|x| x.trim()).collect::<Vec<_>>();
    (format!("{} {}",keys[0],keys[1]), parse_contents(parsed_line[1]))
}

fn parse_input(input: &str) -> HashMap<String, HashMap<String, i32>> {
    input.split('\n').map(|line| parse_line(line)).collect()
}