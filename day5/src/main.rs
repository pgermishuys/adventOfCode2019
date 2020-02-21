use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs;

/// An Intcode Interpreter is a virtual machine that uses opcodes
/// to modify its internal memory
pub struct IntcodeInterpreter {
    memory: Vec<isize>,
    ip: usize,
}

impl IntcodeInterpreter {
    pub fn new(memory: Vec<isize>) -> Self {
        Self { memory, ip: 0 }
    }

    /// Sets a memory address to a value
    pub fn set(&mut self, position: usize, value: isize) {
        self.memory[position] = value;
    }

    /// Reads from a memory address
    pub fn get(&self, position: usize) -> isize {
        self.memory[position]
    }

    /// Shows the memory for debugging
    pub fn print(&self) {
        println!("{:?}", self.memory);
    }

    /// Get the current instruction
    pub fn current_instruction(&self) -> isize {
        self.get(self.ip) % 100
    }

    /// Runs the program in memory until the stopcode (99) is reached
    ///
    /// All new ops should have their own method.
    /// They take no rust args, but read in args as needed and
    /// shift the instruction pointer when they're done.
    /// Steps should be the number of args used + 1 for the opcode
    pub fn run(&mut self) {
        loop {
            match self.current_instruction() {
                1 => self.op1(),
                2 => self.op2(),
                3 => self.op3(),
                4 => self.op4(),
                5 => self.op5(),
                6 => self.op6(),
                7 => self.op7(),
                8 => self.op8(),
                99 => return,
                _ => panic!("Unrecognized opcode {}.", self.get(self.ip)),
            };
        }
    }

    /// Reads a number from STDIN
    fn read_stdin() -> isize {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("STDIN read failed.");
        buffer.trim().parse::<isize>().unwrap()
    }

    /// Write a number to STDOUT
    fn write_stdout(number: isize) {
        println!("{}", number);
    }

    /// Process the parameter mode and provide the value given
    /// as a parameter
    fn arg(&self, offset: usize) -> isize {
        let new_index = (self.ip + offset) % self.memory.len();
        let mode = (self.memory[self.ip] / 10isize.pow(1 + offset as u32)) % 2;
        if mode == 1 {
            self.memory[new_index]
        } else if mode == 0 {
            self.get(self.memory[new_index] as usize)
        } else {
            panic!("Unknown parameter mode {}", mode);
        }
    }

    /// Returns the address to write output to
    fn output_address(&self, offset: usize) -> usize {
        let new_index = (self.ip + offset) % self.memory.len();
        self.memory[new_index] as usize
    }

    /// Steps the IP forward "count" steps, wrapping if needed
    fn step(&mut self, count: usize) {
        self.ip = (self.ip + count) % self.memory.len();
    }

    /// Add [1] + [2], store in [3]
    fn op1(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);

        self.set(out, in1 + in2);

        self.step(4);
    }

    /// Mult [1] * [2], store in [3]
    fn op2(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);

        self.set(out, in1 * in2);

        self.step(4);
    }

    /// Read one value from STDIN and store it in [1]
    fn op3(&mut self) {
        let out = self.output_address(1);

        self.set(out, Self::read_stdin());

        self.step(2);
    }

    /// Read [1] and send it to STDOUT
    fn op4(&mut self) {
        Self::write_stdout(self.arg(1));

        self.step(2);
    }

    /// If [1] != 0, set IP -> [2], else nothing
    fn op5(&mut self) {
        if self.arg(1) != 0 {
            self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    /// if [1] == 0, set IP -> [2], else nothing
    fn op6(&mut self) {
        if self.arg(1) == 0 {
            self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    /// if [1] < [2], set [3] to 1, else 0
    fn op7(&mut self) {
        let out = self.output_address(3);

        if self.arg(1) < self.arg(2) {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }

        self.step(4);
    }

    /// if [1] == [2], set [3] to 1, else 0
    fn op8(&mut self) {
        let out = self.output_address(3);

        if self.arg(1) == self.arg(2) {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }

        self.step(4);
    }
}

fn run_program(mut memory: std::vec::Vec<i32>, mut pointer: i32) -> std::vec::Vec<i32> {
    let instruction = format!("{:0>4}", memory[pointer as usize].to_string());
    let opcode = instruction[2..].parse::<i32>().unwrap();
    println!("Memory: {:?}", memory);
    println!("Pointer: {:?}", pointer);
    println!("Memory at Pointer: {:?}", memory[pointer as usize]);
    println!("Instruction: {:?}", &instruction);
    println!("Opcode: {:?}", opcode);
    println!("Press any key to continue...");
    let mut wait = String::new();
    io::stdin()
        .read_line(&mut wait)
        .expect("error: unable to read user input");
    if opcode == 99 {
        return memory;
    }
    if opcode == 1 {
        let mode = get_mode(&instruction, 1);
        let first_value = get_input_for_mode(mode, pointer + 1, &memory);
        let mode = get_mode(&instruction, 0);
        let second_value = get_input_for_mode(mode, pointer + 2, &memory);
        let result_address = memory[(pointer + 3) as usize];
        println!(
            "OpCode(1) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
            first_value, second_value, result_address
        );
        memory[result_address as usize] = first_value + second_value;
        pointer = pointer + 4;
    }
    if opcode == 2 {
        let mode = get_mode(&instruction, 1);
        let first_value = get_input_for_mode(mode, pointer + 1, &memory);
        let mode = get_mode(&instruction, 0);
        let second_value = get_input_for_mode(mode, pointer + 2, &memory);
        let result_address = memory[(pointer + 3) as usize];
        // println!(
        //     "OpCode(2) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
        //     first_value, second_value, result_address
        // );
        memory[result_address as usize] = first_value * second_value;
        pointer = pointer + 4;
    }
    if opcode == 3 {
        println!("Please enter the input for the computer:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let result_address = get_input_for_mode(0, input.trim().parse::<i32>().unwrap(), &memory);
        // println!(
        //     "OpCode(3) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
        //     first_value, second_value, result_address
        // );
        memory[result_address as usize] = input.trim().parse::<i32>().unwrap();
        pointer = pointer + 2;
    }
    if opcode == 4 {
        let mode = get_mode(&instruction, 1);
        let first_value = get_input_for_mode(mode, pointer + 1, &memory);
        // println!(
        //     "OpCode(4) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
        //     first_value, second_value, result_address
        // );
        println!("Output: {:?}", first_value);
    }
    // println!("Memory: {:?}", memory);
    return run_program(memory, pointer);
}
fn get_mode(instruction: &String, index: usize) -> u32 {
    let mode = instruction
        .chars()
        .nth(index)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap();
    println!("Instruction: {:?}, Mode: {:?}", instruction, mode);
    return mode;
}
fn get_input_for_mode(mode: u32, index: i32, memory: &std::vec::Vec<i32>) -> i32 {
    if mode == 0 {
        return memory[memory[(index) as usize] as usize];
    }
    return memory[index as usize];
}
fn patch_memory(
    index_one: usize,
    input_one: i32,
    index_two: usize,
    input_two: i32,
    mut memory: std::vec::Vec<i32>,
) -> std::vec::Vec<i32> {
    memory[index_one] = input_one;
    memory[index_two] = input_two;
    return memory;
}
fn put_into_memory(input: &String) -> std::vec::Vec<isize> {
    let mut memory: Vec<isize> = Vec::new();
    for line in input.lines() {
        let opcodes: Vec<&str> = line.split(',').collect();
        for opcode in opcodes {
            memory.push(opcode.parse::<isize>().unwrap());
        }
    }
    return memory;
}
fn read_memory_from_file() -> String {
    let file_name = "memory.txt";
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer);
    return buffer;
}
fn main() {
    let memory_from_file = read_memory_from_file();
    let memory = put_into_memory(&memory_from_file);
    let mut interpreter = IntcodeInterpreter::new(memory);
    interpreter.run();
    // println!("Memory (Before): {:?}", memory);
    // let memory = patch_memory(1, 12, 2, 2, memory);
    // let memory = run_program(memory, 0);
    // println!("Memory (After): {:?}", memory);
}
