use crate::input;


static PASSPORT_FIELDS:[&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
static COLORS:[&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn valid_passports() -> i32{
    let passports: Vec<Vec<Vec<&str>>> = input::_INPUT.split("\n\n")
        .map(|passport| {
            passport.trim().split(&[' ', '\n'][..]).map(|field| {
                field.trim().split(':').collect()
            }).collect()
        })
        .collect();
    
    let mut counter = 0;
    for passport in passports {
        let mut field_map = Vec::new();
        for field in passport{
            if !field_map.contains(&field[0]) {
                if validate_field(field[0], field[1]){
                    field_map.push(field[0]);
                }
            }else{
                field_map.pop();
                field_map.pop();
                field_map.pop();
                field_map.pop();
                break;
            }
        }
        if field_map.len() == 7{
            counter += 1;
        }
    }
    counter
}

fn validate_field(name:&str, value: &str) -> bool {
    if PASSPORT_FIELDS.contains(&name){
        match name {
            "byr" => {
                let parsed_value = value.parse::<i32>().unwrap();
                if parsed_value >= 1920 && parsed_value <= 2002 {
                    true
                }else{
                    false
                }
            }, 
            "iyr" => {
                let parsed_value = value.parse::<i32>().unwrap();
                if parsed_value >= 2010 && parsed_value <= 2020 {
                    true
                }else{
                    false
                }
            }, 
            "eyr" => {
                let parsed_value = value.parse::<i32>().unwrap();
                if parsed_value >= 2020 && parsed_value <= 2030 {
                    true
                }else{
                    false
                }
            }, 
            "hgt" => {
                if value.len() <= 2 {return false}
                let unit = &value[value.len()-2..value.len()];
                let length = (&value[0..value.len()-2]).parse::<i32>().unwrap();
                match unit {
                    "cm" if length >= 150 && length <= 193 => true,
                    "in" if length >= 59 && length <= 76   => true,
                    _                                      => false
                }
            }, 
            "hcl" => {
                if value.len() != 7 {return false}
                let hex = i32::from_str_radix(&value[1..value.len()], 16);
                let first = &value[0..1];
                match hex {
                    Result::Ok(_) if first == "#"  => true,
                    _                              => false
                }
            }, 
            "ecl" => {
                if COLORS.contains(&value) {true} else {false}
            }, 
            "pid" => {
                let result = value.parse::<i32>();
                match result{
                    Result::Ok(_) if value.len() == 9 => true,
                    _                                 => false
                }
            },
            _     => false
        }
    }else{
        false
    }      
}