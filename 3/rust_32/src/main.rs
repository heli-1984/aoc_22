fn main() {
    let data = include_str!("input.txt");
    let mut score = 0;
    let mut loop_int = 0;
    while loop_int < data.lines().count() - 1 {
        if loop_int % 3 == 0 {
            for char in data.lines().nth(loop_int).unwrap().chars() {
                if data.lines().nth(loop_int + 1).unwrap().contains(char)
                    && data.lines().nth(loop_int + 2).unwrap().contains(char)
                {
                    println!("{char}");
                    score += scorer(char);
                    break;
                }
            }
        }
        loop_int += 1;
    }
    println!("{score}");
}

fn scorer(char: char) -> i32 {
    if char.is_uppercase() {
        return (char.to_digit(36).unwrap() + 17) as i32;
    } else {
        return (char.to_digit(36).unwrap() - 9) as i32;
    }
}
