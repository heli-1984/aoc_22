fn main() {
    let data = include_str!("input.txt");
    let mut total = 0;
    for line in data.lines() {
        let part1 = line.split(',').nth(0).unwrap();
        let part2 = line.split(',').nth(1).unwrap();
        let mut range1: Vec<i32> = [].to_vec();
        let mut range2: Vec<i32> = [].to_vec();
        for i in part1.split('-').nth(0).unwrap().parse::<i32>().unwrap()
            ..part1.split('-').nth(1).unwrap().parse::<i32>().unwrap() + 1
        {
            range1.push(i);
        }
        for i in part2.split('-').nth(0).unwrap().parse::<i32>().unwrap()
            ..part2.split('-').nth(1).unwrap().parse::<i32>().unwrap() + 1
        {
            range2.push(i);
        }
        for i in range1 {
            if range2.contains(&i) {
                total += 1;
                break;
            }
        }
    }
    println!("Total: {}", total);
}
