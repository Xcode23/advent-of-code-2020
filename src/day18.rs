use crate::input;
use Token::*;

static INPUT: &str = input::_INPUT;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Open
}

pub fn parse_math_part1() -> i64 {
    INPUT.lines().map(|line| {
        let mut postfix: Vec<Token> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();
        let expr = line.trim().chars().filter(|symbol| *symbol != ' ');
        for symbol in expr {
            match symbol {
                '0'..='9'   =>  { postfix.push(Num(symbol.to_string().parse::<i64>().unwrap())); }
                '+'|'*'     =>  { 
                                    while !operators.is_empty() && *operators.last().unwrap() != Open {
                                        postfix.push(operators.pop().unwrap());
                                    } 
                                    if symbol == '+' {
                                        operators.push(Add);
                                    } else {
                                        operators.push(Mul);
                                    }
                                }
                '('         =>  { operators.push(Open); }
                ')'         =>  { 
                                    while *operators.last().unwrap() != Open {
                                        postfix.push(operators.pop().unwrap());
                                    }
                                    operators.pop();
                                }
                _           =>  {panic!("What the fuck is this? : {}\n", symbol);}   
            }
        }
        while !operators.is_empty() {
            postfix.push(operators.pop().unwrap());
        }
        eval_postfix(postfix)
    }).sum()
}

pub fn parse_math_part2() -> i64 {
    INPUT.lines().map(|line| {
        let mut postfix: Vec<Token> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();
        let expr = line.trim().chars().filter(|symbol| *symbol != ' ');
        for symbol in expr {
            match symbol {
                '0'..='9'   =>  { postfix.push(Num(symbol.to_string().parse::<i64>().unwrap())); }
                '+'|'*'     =>  { 
                                    while !operators.is_empty() && *operators.last().unwrap() == Add {
                                        postfix.push(operators.pop().unwrap());
                                    } 
                                    if symbol == '+' {
                                        operators.push(Add);
                                    } else {
                                        operators.push(Mul);
                                    }
                                }
                '('         =>  { operators.push(Open); }
                ')'         =>  { 
                                    while *operators.last().unwrap() != Open {
                                        postfix.push(operators.pop().unwrap());
                                    }
                                    operators.pop();
                                }
                _           =>  {panic!("What the fuck is this? : {}\n", symbol);}   
            }
        }
        while !operators.is_empty() {
            postfix.push(operators.pop().unwrap());
        }
        eval_postfix(postfix)
    }).sum()
}

fn eval_postfix(mut postfix: Vec<Token>) -> i64 {
    postfix.reverse();
    let mut stack = Vec::new();
    while !postfix.is_empty() {
        match postfix.pop().unwrap() {
            Num(value) => {stack.push(value)}
            Add =>  { 
                        let operand1 = stack.pop().unwrap();
                        let operand2 = stack.pop().unwrap();
                        stack.push(operand1 + operand2);
                    }
            Mul =>  { 
                        let operand1 = stack.pop().unwrap();
                        let operand2 = stack.pop().unwrap();
                        stack.push(operand1 * operand2);
                    }
            _   => {}
        }
    }
    stack.pop().unwrap()
}