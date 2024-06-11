use std::{env, fs};

const RED_C: u32 = 12;
const GREEN_C: u32 = 13;
const BLUE_C: u32 = 14;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = content.lines();

    let mut sum = 0;
    let mut cube_sum = 0;
    for game in lines {
        sum += parse_game_ex_1(game);
        cube_sum += parse_game_ex_2(game);
    }

    println!("RESULT: {}, CUBE SUM: {}", sum, cube_sum);
}

fn parse_game_ex_1(game: &str) -> u32 {
    let mut game_iter = game.split(':');

    let game_id = game_iter.next();

    let rounds: Vec<&str> = game_iter.next().unwrap_or("").split(';').collect();

    for r in rounds {
        let items: Vec<&str> = r.split(',').collect();
        for item in items {
            let mut vals = item.trim().split(' ');
            let count: u32 = vals.next().unwrap_or("0").parse().unwrap_or(0);
            let name: &str = vals.next().unwrap_or("");
            match name {
                "red" => {
                    if count > RED_C {
                        return 0;
                    }
                }
                "green" => {
                    if count > GREEN_C {
                        return 0;
                    }
                }
                "blue" => {
                    if count > BLUE_C {
                        return 0;
                    }
                }
                &_ => (),
            }
        }
    }

    game_id
        .unwrap_or("Game 0")
        .strip_prefix("Game ")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

fn parse_game_ex_2(game: &str) -> u32 {
    let mut game_iter = game.split(':');

    let mut r_max: Option<u32> = None;
    let mut g_max: Option<u32> = None;
    let mut b_max: Option<u32> = None;

    let _game_id = game_iter.next();

    let rounds: Vec<&str> = game_iter.next().unwrap_or("").split(';').collect();

    for r in rounds {
        let items: Vec<&str> = r.split(',').collect();
        for item in items {
            let mut vals = item.trim().split(' ');
            let count: u32 = vals.next().unwrap_or("0").parse().unwrap_or(0);
            let name: &str = vals.next().unwrap_or("");
            match name {
                "red" => {
                    if r_max.is_none() || Some(count) > r_max {
                        r_max = Some(count);
                    }
                }
                "green" => {
                    if g_max.is_none() || Some(count) > g_max {
                        g_max = Some(count);
                    }
                }
                "blue" => {
                    if b_max.is_none() || Some(count) > b_max {
                        b_max = Some(count);
                    }
                }
                &_ => (),
            }
        }
    }

    r_max.unwrap_or(1) * g_max.unwrap_or(1) * b_max.unwrap_or(1)
}
