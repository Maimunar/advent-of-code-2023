use std::env;
use std::fs;

fn ex1() {
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

const DIGITS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = content.lines();

    let mut sum: u32 = 0;

    for line in lines {
        let mut first_i = line.find(char::is_numeric).unwrap_or(line.len() - 1);
        let mut last_i = line.rfind(char::is_numeric).unwrap_or(0);
        let mut first: u32 = (line.as_bytes()[first_i] as char).to_digit(10).unwrap_or(0);
        let mut last: u32 = (line.as_bytes()[last_i] as char).to_digit(10).unwrap_or(0);

        for (name, digit) in DIGITS {
            // First
            let digit_i = line.find(name);
            if let Some(i) = digit_i {
                if first_i > i {
                    first_i = i;
                    first = digit as u32;
                }
            };

            // Last
            let digit_i = line.rfind(name);
            if let Some(i) = digit_i {
                if last_i < i {
                    last_i = i;
                    last = digit as u32;
                }
            };
        }

        sum += first * 10 + last;
    }

    println!("Sum: {}", sum);
}
