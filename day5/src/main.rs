use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::*;

fn run_program(mut memory: std::vec::Vec<i32>, mut pointer: i32) -> std::vec::Vec<i32> {
    let instruction: i32 = memory[pointer as usize];
    let opcode = instruction % 1000;
    println!("Opcode: {:?}", opcode);
    if opcode == 99 {
        return memory;
    }
    if opcode == 1 {
        let mode = get_mode(instruction, 1);
        let first_value = get_input_for_mode(mode, pointer + 1, &memory);
        let mode = get_mode(instruction, 0);
        let second_value = get_input_for_mode(mode, pointer + 2, &memory);
        let result_address = memory[(pointer + 3) as usize];
        // println!(
        //     "OpCode(1) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
        //     first_value, second_value, result_address
        // );
        memory[result_address as usize] = first_value + second_value;
        pointer = pointer + 4;
    }
    if opcode == 2 {
        let mode = get_mode(instruction, 1);
        let first_value = get_input_for_mode(mode, pointer + 1, &memory);
        let mode = get_mode(instruction, 0);
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
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let mode = get_mode(instruction, 1);
        let result_address = get_input_for_mode(mode, input.parse::<i32>().unwrap(), &memory);
        // println!(
        //     "OpCode(3) - first_value: {:?}, second_value: {:?}, result_address: {:?}",
        //     first_value, second_value, result_address
        // );
        memory[result_address as usize] = input.trim().parse::<i32>().unwrap();
        pointer = pointer + 2;
    }
    if opcode == 4 {
        let mode = get_mode(instruction, 1);
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
fn get_mode(instruction: i32, index: usize) -> u32 {
    let mode = instruction
        .to_string()
        .chars()
        .nth(index)
        .unwrap()
        .to_digit(10)
        .unwrap();
    println!("Mode: {:?}", mode);
    return mode;
}
fn get_input_for_mode(mode: u32, index: i32, memory: &std::vec::Vec<i32>) -> i32 {
    if mode == 0 {
        println!("Index: {:?}", index);
        println!("Memory: {:?}", memory);
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
fn put_into_memory(input: &String) -> std::vec::Vec<i32> {
    let mut memory: Vec<i32> = Vec::new();
    for line in input.lines() {
        let opcodes: Vec<&str> = line.split(',').collect();
        for opcode in opcodes {
            memory.push(opcode.parse::<i32>().unwrap());
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
    println!("Memory (Before): {:?}", memory);
    // let memory = patch_memory(1, 12, 2, 2, memory);
    let memory = run_program(memory, 0);
    println!("Memory (After): {:?}", memory);
}
