use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut value;
    let mut highest_value = 0;
    let mut highest_index = 0;
    let mut index = 0;
    for i in data.split("\n\n") {
        index += 1;
        value = 0;
        for z in i.split("\n") {
            value += z.parse::<i32>().unwrap_or(0);
        }
        if value > highest_value {
            highest_value = value;
            highest_index = index;
        }
    }
    println!("Highest calorie count: {} at {}", highest_value, highest_index);
}
