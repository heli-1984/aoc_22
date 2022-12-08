fn main() {
    let data = include_str!("input.txt");
    let mut amount = 0;
    for line in data.lines() {
        let range1 = line.split(',').next().unwrap();
        let range2 = line.split(',').nth(1).unwrap();
        if range1.split('-').next().unwrap().parse::<i32>().unwrap()
            <= range2.split('-').next().unwrap().parse::<i32>().unwrap()
            && range1.split('-').nth(1).unwrap().parse::<i32>().unwrap()
                >= range2.split('-').nth(1).unwrap().parse::<i32>().unwrap()
        {
            amount += 1;
        } else if range1.split('-').next().unwrap().parse::<i32>().unwrap()
            >= range2.split('-').next().unwrap().parse::<i32>().unwrap()
            && range1.split('-').nth(1).unwrap().parse::<i32>().unwrap()
                <= range2.split('-').nth(1).unwrap().parse::<i32>().unwrap()
        {
            amount += 1;
        }
    }
    println!("Total: {}", amount);
}
