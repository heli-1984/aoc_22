use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut value;
    let mut values = Vec::new();
    for i in data.split("\n\n") {
        value = 0;
        for z in i.split("\n") {
            value += z.parse::<i32>().unwrap_or(0);
        }
        values.push(value);
    }
    values.sort();
    values.reverse();
    let mut total_3 = 0;
    for i in 0..3 {
        total_3 += values[i];
    }
    println!("{total_3}");
}
