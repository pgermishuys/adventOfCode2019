use math::round;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn calculate_module_fuel(module_weight: i64) -> i64 {
    return round::floor((module_weight / 3) as f64, 2) as i64 - 2;
}
fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let module_weight = line.parse::<i64>().unwrap();
        total += calculate_module_fuel(module_weight);
        println!("{}", module_weight);
    }
    println!("total: {}", total);
}
