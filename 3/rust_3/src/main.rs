fn main() {
    let data = include_str!("input.txt");
    let mut score = 0;
    for line in data.lines() {
        if line != "" {
            let halfway_point = line.chars().count() / 2;
            let halves = line.split_at(halfway_point);
            for character in halves.0.chars() {
                if halves.1.contains(character) {
                    score += scorer(character);
                    break;
                }
            }
        }
    }
    println!("Total score {}", score);
}

fn scorer(char: char) -> i32 {
    if char.is_uppercase() {
        return (char.to_digit(36).unwrap() + 17) as i32;
    } else {
        return (char.to_digit(36).unwrap() - 9) as i32;
    }
}
