use math::round;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn calculate_module_fuel(module_weight: i64) -> i64 {
    return round::floor((module_weight / 3) as f64, 2) as i64 - 2;
}

fn calculate_module_fuel_rec(mut total: i64, module_weight: i64) -> i64 {
    if calculate_module_fuel(module_weight) <= 0 {
        return total;
    }
    let remainder = calculate_module_fuel(module_weight);
    if remainder <= 0 {
        return total;
    }
    total += remainder;
    return calculate_module_fuel_rec(total, remainder);
}

fn main() {
    let file_name_part1 = "part1.txt";
    let file_part1 = File::open(file_name_part1).unwrap();
    let reader_part1 = BufReader::new(file_part1);

    let mut total_part1 = 0;
    for (_, line) in reader_part1.lines().enumerate() {
        let line = line.unwrap();
        let module_weight = line.parse::<i64>().unwrap();
        total_part1 += calculate_module_fuel(module_weight);
    }
    println!("Part One: {}", total_part1);

    let file_name_part2 = "part2.txt";
    let file_part2 = File::open(file_name_part2).unwrap();
    let reader_part2 = BufReader::new(file_part2);

    let mut total_part2 = 0;
    for (_, line) in reader_part2.lines().enumerate() {
        let line = line.unwrap();
        let module_weight = line.parse::<i64>().unwrap();
        total_part2 += calculate_module_fuel_rec(0, module_weight);
    }
    println!("Part Two: {}", total_part2);
}
