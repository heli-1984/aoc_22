fn main() {
    let data = include_str!("input.txt");
    let mut stack:Vec<String> = [].to_vec();
    let length = data.lines().nth(0).unwrap().chars().count() / 4;
    let mut height = 0;
    for line in data.lines() {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }
        height += 1;
    }
    for n in 0..length + 1 {
        let mut temp = "".to_string();
        for i in 0..height {
            let line = data.lines().nth(i).unwrap();
            if line.chars().nth(0 + (4 * n)).unwrap() != ' ' {
                temp.push(line.chars().nth(1 + (4 * n)).unwrap());
            } 
        }
        stack.push(temp.chars().rev().collect::<String>().clone());
    }
    for line in data.lines() {
        let mut instruction: Vec<usize> = [].to_vec();
        if line != "" {
            if line.split_whitespace().nth(0).unwrap() == "move" {
                for b in line.split_whitespace() {
                    if b.contains("move") || b.contains("from") || b.contains("to") {

                    } else {
                        instruction.push(b.parse::<usize>().unwrap());
                    }
                }
            }
        }
        if instruction != [] {
            for _ in 0..instruction[0] {
                let popped = stack[instruction[1] - 1].pop().unwrap();
                stack[instruction[2] - 1].push(popped);
            }
        }
    }
    println!("{:?}", stack);
}
