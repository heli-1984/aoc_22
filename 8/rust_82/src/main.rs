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
    let mut scenic_score = vec![0; length*height];
    //left to right check
    for row in trees.clone() {
        for column in 0..length {
            current_tree += 1;
            let mut temp_vec = Vec::new();
            visible = 0;
            //print!("{} ", row[column]);
            for test in column + 1..length {
                temp_vec.push(row[test]);
                if row[test] < row[column] {
                    if *temp_vec.first().unwrap() != 0 {
                        visible += 1;
                    }
                } else {
                    visible += 1;
                    break;
                }
            }
            scenic_score[current_tree - 1] = visible;
            //println!("{:?}", temp_vec);
            //println!("visible: {}", visible);
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
            //let mut temp_vec = Vec::new();
            //print!("row[column]: {} ", row[column]);
            for test in (0..column).rev() {
                //print!("row[test]: {} ", row[test]);
                if row[test] < row[column] {
                    if row[test] != 0 {
                        visible += 1;
                    }
                    } else {
                        visible += 1;
                        break;
                    }
                }
                //println!("visible: {visible}");
                scenic_score[current_tree - 1] *= visible;
            }
        }
    //println!("{:?}", trees_seen);
    println!("{:?}", scenic_score);
}
