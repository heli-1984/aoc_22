use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut score = 0;
    for i in data.split("\n") {
        if i != "" {
            score += win(i.split_whitespace().nth(0).unwrap(), i.split_whitespace().nth(1).unwrap());
            if i.split_whitespace().nth(1).unwrap() == "X" {
                score += 1;
            }
            if i.split_whitespace().nth(1).unwrap() == "Y" {
                score += 2;
            }
            if i.split_whitespace().nth(1).unwrap() == "Z" {
                score += 3;
            }
        }
    }
    println!("{}", score);
}

fn win(player1: &str, player2: &str) -> i32 {
    if player1 == "A" {
        if player2 == "Y" {
            return 6;
        } else if player2 == "X" {
            return 3;
        } else {
            return 0;
        }
    } else if player1 == "B" {
        if player2 == "Z" {
            return 6;
        } else if player2 == "Y" {
            return 3;
        } else {
            return 0;
        }
    } else if player1 == "C" {
        if player2 == "X" {
            return 6;
        } else if player2 == "Z" {
            return 3;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}
