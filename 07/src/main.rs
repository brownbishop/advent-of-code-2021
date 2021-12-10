fn part1(positions: &Vec<i64>) {
    let max_pos: i64 = *positions.iter().max().unwrap();
    // fuel_consumption for given position
    let fuel_consumption = |i: i64| positions.iter()
                                    .map(|x| (x - i).abs())
                                    .sum();
    let all_positions = 0..(max_pos + 1);
    let optimal_fuel: i64 = all_positions.map(fuel_consumption)
        .min()
        .unwrap();
    println!("{}", optimal_fuel);
}

fn fuel_consumption2(pos1: i64, pos2: i64) -> i64 {
    // now the cost of the fuel equals the Gaussian sum
    // for n = distance + 1
    let n = (pos1 - pos2).abs();
    n * (n + 1) / 2
}

fn part2(positions: &Vec<i64>) {
    let max_pos: i64 = *positions.iter().max().unwrap();
    // fuel_consumption for given position
    let fuel_consumption = |i: i64| positions.iter()
                                    .map(|x| fuel_consumption2(*x, i))
                                    .sum();
    let all_positions = 0..(max_pos + 1);
    let optimal_fuel: i64 = all_positions.map(fuel_consumption)
        .min()
        .unwrap();
    println!("{}", optimal_fuel);
}

fn main() {
    let crab_positions: Vec<i64> = include_str!("input").trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    part1(&crab_positions);
    part2(&crab_positions);
}
