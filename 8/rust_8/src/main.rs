fn main() {
    let data = include_str!("test.txt");
    let length = data.lines().nth(0).unwrap().chars().count();
    let height = data.lines().count();
    let mut trees = Vec::new();
    println!("Height: {height}, length: {length}");
    println!("{data}");
    for line in data.lines() {
        let mut temp_vec: Vec<usize> = [].to_vec();
        for char in line.chars() {
            temp_vec.push(char.to_digit(10).unwrap() as usize);
        }
        trees.push(temp_vec);
    }
    println!("{:?}", trees);
    for part in trees {
        for part2 in part {
            print!("{part2}");
        }
        println!();
    }
}
