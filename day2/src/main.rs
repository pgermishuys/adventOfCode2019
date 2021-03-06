use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn process_program(mut program: Vec<i64>, program_pointer: usize) -> Vec<i64> {
    if program[program_pointer] == 99 {
        return program;
    } else {
        program = process_opcode(program, program_pointer);
        return process_program(program, program_pointer + 4);
    }
}

fn process_opcode(mut program: Vec<i64>, program_pointer: usize) -> Vec<i64> {
    let opcode = program[program_pointer];
    let first_operand = program[program_pointer + 1] as usize;
    let second_operand = program[program_pointer + 2] as usize;
    let result_index = program[program_pointer + 3] as usize;
    if opcode == 1 {
        program[result_index] = program[first_operand] + program[second_operand];
    }
    if opcode == 2 {
        program[result_index] = program[first_operand] * program[second_operand];
    }
    return program;
}

fn patch_program(mut program: Vec<i64>, noun: i64, verb: i64) -> Vec<i64> {
    program[1] = noun;
    program[2] = verb;
    return program;
}

fn read_program() -> Vec<i64> {
    let file_name_part1 = "part1.txt";
    let file_part1 = File::open(file_name_part1).unwrap();

    let reader_part1 = BufReader::new(file_part1);
    let mut program: Vec<i64> = Vec::new();
    for (_, line) in reader_part1.lines().enumerate() {
        let line = line.unwrap();
        for item in line.split(",") {
            program.push(item.parse::<i64>().unwrap());
        }
    }
    return program;
}

fn main() {
    println!(
        "Part One: {:?}",
        process_program(patch_program(read_program(), 12, 2), 0)
    );
    for noun in 0..99 {
        for verb in 0..99 {
            let result = process_program(patch_program(read_program(), noun, verb), 0);
            println!("Part Two: {:?}", result);
            if result[0] == 19690720 {
                println!(
                    "Answer found. Noun: {}, Verb: {}. (100 * Verb) + Noun = {}",
                    noun,
                    verb,
                    (100 * noun) + verb
                );
                return;
            }
        }
    }
}
