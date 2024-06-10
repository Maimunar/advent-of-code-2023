use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = content.lines();

    let mut sum: u32 = 0;

    for line in lines {
        let first_digit_i = line.find(char::is_numeric).unwrap_or(0);
        let last_digit_i = line.rfind(char::is_numeric).unwrap_or(0);

        let first_d = line.as_bytes()[first_digit_i] as char;
        let last_d = line.as_bytes()[last_digit_i] as char;

        let first_d: u32 = first_d.to_digit(10).unwrap_or(0);
        let last_d: u32 = last_d.to_digit(10).unwrap_or(0);

        let d = first_d * 10 + last_d;

        sum += d;
    }

    println!("Sum: {}", sum);
}
