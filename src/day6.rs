use crate::input;
use std::{iter::FromIterator, collections::HashSet};
use std::collections::HashMap;

pub fn unique_question_count() -> i32{
    let questions_per_group = input::_INPUT.split("\n\n")
        .map(|group| {
            let group = group.trim().split('\n').map(|person| {
                person.trim()
            });
            let mut questions = HashSet::new();
            for person in group {
                for question in person.chars() {
                    questions.insert(question);
                }
            }
            questions.len() as i32
        });
    questions_per_group.sum() 
}

pub fn all_question_count() -> i32{
    let questions_per_group: Vec<_> = input::_INPUT.split("\n\n")
        .map(|group| {
            let parsed_group = group.trim().split('\n').map(|person| {
                person.trim()
            });
            let mut current_questions = HashSet::new();
            let mut previous_questions: HashSet<char> = FromIterator::from_iter('a'..='z');
            for person in parsed_group {
                for question in person.chars() {
                    if previous_questions.contains(&question) {
                        current_questions.insert(question);
                    }
                }
                previous_questions = current_questions;
                current_questions = HashSet::new();
            }
            previous_questions.len() as i32
        }).collect();
    questions_per_group.iter().sum() 
}













