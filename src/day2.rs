use crate::input;

pub fn valid_passwords(policy_type: i32) -> i32 {
    let passwords = input::_INPUT.split('\n').map(|pass| {
        pass.trim().split(':').map(|x| x.trim()).collect::<Vec<_>>()
    });
    let mut counter = 0;
    for password in passwords {
        let policy = password[0].split(&['-', ' '][..]).map(|x| x.trim()).collect::<Vec<_>>();
        if policy_type == 1 && first_policy(&policy, password[1]) {
            counter += 1;
        }
        
        if policy_type == 2 && second_policy(&policy, password[1]) {
            counter += 1;
        }
    }
    counter
}

fn first_policy(policy: &Vec<&str>, password: &str) -> bool {
    let mut char_count = 0;
    for letter in password.chars(){
        if letter == policy[2].chars().next().unwrap() {
            char_count += 1;
        }
    }
    if char_count >= policy[0].parse::<i32>().unwrap() && char_count <= policy[1].parse::<i32>().unwrap() {
        true
    }else {
        false
    }
}

fn second_policy(policy: &Vec<&str>, password: &str) -> bool {
    let vec_chars: Vec<char> = password.chars().collect();
    let first_index = policy[0].parse::<usize>().unwrap() - 1;
    let second_index = policy[1].parse::<usize>().unwrap() - 1;
    let policy_char = policy[2].chars().next().unwrap();
    
    if (vec_chars[first_index] == policy_char) != (vec_chars[second_index] == policy_char) {
        true
    }else {
        false
    }
}