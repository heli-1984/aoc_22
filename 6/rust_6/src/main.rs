fn main() {
    let data = include_str!("input.txt");
    let mut previous4 = data[0..4].to_string();
    let mut counter = 4;
    for char in data[4..data.chars().count()].chars() {
        let mut correct = true;
        for i in 0..4 {
            let mut temp = previous4.clone();
            temp.remove(i);
            if previous4.chars().all(|x| temp.contains(x)) {
                correct = false;
            }
        }
        if correct {
            println!("{counter}");
            break;
        }
        if previous4.chars().count() == 4 {
            previous4.remove(0);
            previous4.push(char);
        }
        counter += 1;
    }
}
