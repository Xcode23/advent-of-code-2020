use std::collections::BTreeMap;
use num::Integer;
use crate::input;


pub fn next_bus_part1() -> i32 {
    let mut data = input::_INPUT.lines();
    let next_possible: i32 = data.next().unwrap().parse().unwrap();
    let available_buses: Vec<i32> = data.next().unwrap().split(',').filter(|bus| *bus != "x").map(|bus| bus.trim().parse().unwrap()).collect();
    let mut leave_time = next_possible;
    let chosen_bus: i32;
    'outer: loop {
        for bus in available_buses.iter() {
            if leave_time % bus == 0 {
                chosen_bus = *bus;
                break 'outer;
            }
        }
        leave_time += 1;
    }
    (leave_time - next_possible) * chosen_bus
}

pub fn next_bus_part2() -> i64 {
    let mut data = input::_INPUT.lines();
    data.next();
    let available_buses: BTreeMap<i64, i64> = data.next().unwrap().split(',').enumerate()
    .filter(|(_, bus)| *bus != "x")
    .map(|(index, bus)| (bus.trim().parse().unwrap(), index as i64)).collect();
    let max_bus = *available_buses.keys().max().unwrap();
    let max_index = *available_buses.get(&max_bus).unwrap() as i64;
    let mut timestump = max_bus;
    let mut done_flag = true;
    let mut jump = max_bus;
    loop {
        for bus in available_buses.keys().rev() {
            let true_offset= *available_buses.get(bus).unwrap() - max_index;
            if (timestump + true_offset) % bus == 0 {
                jump = jump.lcm(bus);
            }else{
                done_flag = false;
                break;
            }
        }
        if !done_flag {
            timestump += jump;
            done_flag = true;
        } else {
            break;
        }
    }
    timestump - max_index
}