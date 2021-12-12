fn part1(outputs: Vec<&str>) {
    let mut occurences: u64 = 0;
    let number_lens:[usize; 4] = [2, 3, 4, 7];
    for output in outputs.iter() {
         if number_lens.contains(&output.len()) {
             occurences += 1;
         }
    }

    println!("{}", occurences);
}

// len => number
// 2 => 1,
// 4 => 4,
// 3 => 7,
// 7 => 8,

fn digit_to_set(digit: &str) -> usize {
    let mut set: u8 = 0;
    digit.chars()
        .map(|c| (c as u8) - ('a' as u8))
        .for_each(|c| set += 1 << (c as u8));
    set as usize
}

fn line_to_number(line: &(&str, &str)) -> u64 {
    let mut map: [usize; 10] = [0; 10];
    line.0.split(" ")
        .for_each(|d| {
            let s = digit_to_set(d);
            match d.len() {
                2 => map[1] = s,
                3 => map[7] = s,
                4 => map[4] = s,
                7 => map[8] = s,
                // I don't know how to do non-exhaustive matches in Rust
                _ => {}
            }
        });

    let number: Vec<u64> = line.1.split(" ")
        .map(|d| {
            let s = digit_to_set(d);
            return match d.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    if s & map[1] == map[1] {
                        3
                    } else {
                        if s & (map[4] & !map[7]) == (map[4] & !map[7]){
                            5
                        } else {
                            2
                        }
                    }
                },
                6 => {
                    if s & map[4] == map[4] {
                        9
                    } else {
                        if s & (map[8] & !map[7]) == (map[8] & !map[7]) {
                            6
                        } else {
                            0
                        }
                    }
                },
                _ => 0
            };
        }).collect();
    let num =  number[0] * 1000 + number[1] * 100 + number[2] * 10 + number[3];
    num
}

fn part2(outputs: Vec<(&str, &str)>) {
    let sum: u64 = outputs.iter()
        .map(line_to_number)
        .sum();
    println!("{}", sum);
}

fn main() {
    let input: Vec<(&str, &str)> = include_str!("input").trim()
        .lines()
        // this panics when something goes wrong
        // it's not elegant but it provides fast feedback
        // and this is a small program
        .map(|line| line.split_once(" | ").unwrap())
        .collect();

    // for part 1
    let outputs: Vec<&str> = input.iter()
        .map(|(_, outs)| outs)
        .flat_map(|line| -> Vec<&str> { line.split_whitespace().collect()})
        .collect();

    part1(outputs);

    part2(input);
}

