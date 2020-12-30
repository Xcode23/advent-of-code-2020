use crate::input;
use std::collections::HashSet;
use Instruction::{Nop,Acc,Jmp};



enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

impl Instruction {
    fn new(str_inst:&str, arg: i32) -> Instruction{
        match str_inst {
            "nop" => Ok(Nop(arg)),
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            _     => Err(panic!(""))
        }.unwrap()
    }
}

struct VMachine {
    accumulator: i32,
    ram: Vec<Instruction>,
    pc: i32
}

impl VMachine {
    fn run(&mut self){
        let instruction = &(self.ram[self.pc as usize]);
        match instruction {
            Nop(_) => {}
            Acc(arg) => {self.accumulator += arg;}
            Jmp(arg) => {self.pc += arg - 1;}
            _     => {panic!("bullshit instruction");}
        }
        self.pc += 1;
    }

    //returns true if halt false if loop
    fn run_to_halt_or_loop(&mut self) -> bool{
        let mut verification_set = HashSet::new();
        let mut stuff = (0,1);
        stuff.1 = 2;
        let mut loop_flag = !verification_set.insert(self.pc);
        while !loop_flag && !self.terminated(){
            self.run();
            loop_flag = !verification_set.insert(self.pc);
        }
        !loop_flag
    }

    fn new(program: &str) -> VMachine {
        let parsed_program = program.lines().map(|line| {
            let temp = line.trim().split(' ').collect::<Vec<_>>();
            Instruction::new(temp[0], temp[1].parse().unwrap())
        }).collect();
        VMachine {
            accumulator:0,
            ram: parsed_program,
            pc:0
        }
    } 

    fn terminated(&self) -> bool {
        self.pc >= self.ram.len() as i32
    }

    fn instruction_flip(&mut self, index: usize){
        self.ram[index] = match self.ram[index] {
            Nop(arg) => Jmp(arg),
            Acc(arg) => Acc(arg),
            Jmp(arg) => Nop(arg)
        }
    }
}

pub fn infinite_loop_acc() -> i32 {
    let mut mach = VMachine::new(input::_INPUT);
    let mut verification_set = HashSet::new();
    while verification_set.insert(mach.pc){
        mach.run();
    }
    mach.accumulator
}

pub fn correct_program_acc() -> i32 {
    let mut mach = VMachine::new(input::_INPUT);
    let mut flipped_index = 0;
    mach.instruction_flip(flipped_index);
    while !mach.run_to_halt_or_loop() {
        mach.pc = 0;
        mach.accumulator = 0;
        mach.instruction_flip(flipped_index);
        flipped_index += 1;
        mach.instruction_flip(flipped_index);
    }
    mach.accumulator
}