use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut score = 0;
    for i in data.split("\n") {
        if i != "" {
            let choice = picker(
                i.split_whitespace().nth(0).unwrap(),
                i.split_whitespace().nth(1).unwrap(),
            );
            score += win(i.split_whitespace().nth(0).unwrap(), choice);
            score += match choice {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!("Error"),
            }
        }
    }
    println!("{}", score);
}

fn picker(player1: &str, player2: &str) -> char {
    return match player2 {
        "X" => match player1 {
            "A" => 'Z',
            "B" => 'X',
            "C" => 'Y',
            _ => panic!("Error"),
        },
        "Y" => match player1 {
            "A" => 'X',
            "B" => 'Y',
            "C" => 'Z',
            _ => panic!("Error"),
        },
        "Z" => match player1 {
            "A" => 'Y',
            "B" => 'Z',
            "C" => 'X',
            _ => panic!("Error"),
        },
        _ => panic!("Error"),
    };
}

fn win(player1: &str, player2: char) -> i32 {
    if player1 == "A" {
        if player2 == 'Y' {
            return 6;
        } else if player2 == 'X' {
            return 3;
        } else {
            return 0;
        }
    } else if player1 == "B" {
        if player2 == 'Z' {
            return 6;
        } else if player2 == 'Y' {
            return 3;
        } else {
            return 0;
        }
    } else if player1 == "C" {
        if player2 == 'X' {
            return 6;
        } else if player2 == 'Z' {
            return 3;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}
