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
    
    let mut visible = Vec::new();
    let mut index_h = 0;
    for part in trees.clone() {
        let mut index_l = 0;
        if part != *trees.first().unwrap() && part != *trees.last().unwrap() {
            let mut temp_vec2:Vec<usize> = [part[0]].to_vec(); 
            for part2 in part.clone() {
                let mut index = Vec::new();
                if &part2 > temp_vec2.last().unwrap() && part2 != *part.first().unwrap() && part2 != *part.last().unwrap() {
                    index.push(index_h);
                    index.push(index_l);
                    //println!("{:?}", index);
                    visible.push(index.clone());
                }
                temp_vec2.push(part2);
                temp_vec2.sort();
                index_l += 1;
            }
        }
        index_h += 1;
    }

    index_h = 0;
    for part in trees.clone() {
        let mut part_rev = part.clone();
        part_rev.reverse();
        let mut index_l = 0;
        if part != *trees.first().unwrap() && part != *trees.last().unwrap() {
            let mut temp_vec2:Vec<usize> = [part[length - 1]].to_vec();
            for part2 in part_rev {
                let mut index = Vec::new();
                if &part2 > temp_vec2.last().unwrap() && part2 != *part.first().unwrap() && part2 != *part.last().unwrap() {
                    index.push(index_h);
                    index.push(index_l);
                    //println!("{:?}", index);
                    visible.push(index.clone());
                }
                temp_vec2.push(part2);
                temp_vec2.sort();
                index_l += 1;
            }
        }
        index_h += 1;
    }

    let mut index_l = 0;
    for i in 0..length - 1 {
        let mut index_h = 1;
        if i != 0 {
            let mut temp_vec2:Vec<usize> = [trees[0][i]].to_vec();
            for n in 1..height {
                let mut index = Vec::new();
                if trees[i][n] > *temp_vec2.last().unwrap() && trees[i][n] != trees[i][0] && trees[i][n] != trees[i][length - 1] {
                    index.push(index_h);
                    index.push(index_l);
                    //println!("{:?}", index);
                    visible.push(index);
                }
                temp_vec2.push(trees[n][i]);
                temp_vec2.sort();
                index_h += 1;
            }
        }
        index_l += 1;
    }
    let mut index_l = 0;
    for i in 0..(length - 1) {
        let mut index_h = 0;
        if i != 0 {
            let mut temp_vec2:Vec<usize> = [trees[height - 1][i]].to_vec();
            for n in (1..height).rev() {
                let mut index = Vec::new();
                if trees[i][n] > *temp_vec2.last().unwrap() {
                    index.push(index_h);
                    index.push(index_l);
                    println!("tempvec2: {:?}", temp_vec2);
                    println!("{:?}", index);
                    println!("{i}, {n}");
                    visible.push(index);
                }
                temp_vec2.push(trees[i][n]);
                temp_vec2.sort();
                index_h += 1;
            }
        }
        index_l += 1;
    }

    visible.sort();
    visible.dedup();
    // calculate total visible trees, because all the trees on the outside are visible:
    let total = visible.len() + 2 * length + 2 * (height - 2);
    print!("Total visible trees: {}", total);
}
