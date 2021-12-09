fn solve(mut fishes: [u64; 9], days: u32) {
    // after a day the number of fishes that have to wait x days
    // is basically the number of fishes which had to do that yesterday
    for _ in 0..days {  // Rust cares about ergonomics
        // need more inspiration for a name
        let first = fishes[0];
        for i in 1..9 {
            fishes[i - 1] = fishes[i];
        }
        // newborns wait 8 days
        fishes[8] = first;
        // the old ones wait 6 days,
        // that's what the problem said
        fishes[6] += first;
    }
    let sum: u64 = fishes.iter().sum();
    println!("{}", sum);
}

fn main() {
    // there are 8 days
    // fishes[x] == number of fishes which have to wait x days to multiply
    let fishes: [u64; 9] = include_str!("input").trim()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .fold([0; 9], |mut fish_arr, days_until_multiplication| {
            fish_arr[days_until_multiplication as usize] += 1;
            fish_arr
        });

    // part 1
    solve(fishes, 80);
    // part 2
    solve(fishes, 256);
}
