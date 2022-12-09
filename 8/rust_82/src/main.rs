fn main() {
    let data = include_str!("test.txt");
    let length = data.lines().next().unwrap().chars().count();
    let height = data.lines().count();
    let mut trees = Vec::new();
    for line in data.lines() {
        let mut temp_vec: Vec<usize> = [].to_vec();
        for char in line.chars() {
            temp_vec.push(char.to_digit(10).unwrap() as usize);
        }
        trees.push(temp_vec);
    }
    let mut visible;
    let mut current_tree = 0;
    println!("{}", data);
    let mut scenic_score = vec![0; length * height];
    //left to right check
    for row in trees.clone() {
        for column in 0..length {
            current_tree += 1;
            visible = 0;
            for test in column + 1..length {
                if row[test] < row[column] {
                    if row[test] != 0 {
                        visible += 1;
                    }
                } else {
                    visible += 1;
                    break;
                }
            }
            scenic_score[current_tree - 1] = visible;
        }
    }
    println!("{:?}", scenic_score);
    println!("pt2");
    current_tree = 0;
    //right to left check
    for row in trees.clone() {
        for column in 0..length {
            current_tree += 1;
            visible = 0;
            for test in (0..column).rev() {
                if row[test] < row[column] {
                    if row[test] != 0 {
                        visible += 1;
                    }
                } else {
                    visible += 1;
                    break;
                }
            }
            scenic_score[current_tree - 1] *= visible;
        }
    }
    println!("{:?}", scenic_score);
}
