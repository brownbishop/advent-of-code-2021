fn invalid(vpos: i32, hpos: i32, width: usize, height: usize) -> bool {
    if vpos < 0 || vpos >= height as i32 {
        return true;
    }

    if hpos < 0 || hpos >= width as i32 {
        return true;
    }
    false
}

fn part1(heatmap: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let height = heatmap.len();
    let width = heatmap[0].len();

    let mut sum: u32 = 0;

    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut low_points: Vec<(usize, usize)> = vec![];

    for i in 0..height {
        for j in 0..width {
            let is_lowest = directions.iter()
                .fold(true, |lowest, (dv, dh)| {
                    let vpos = dv + (i as i32);
                    let hpos = dh + (j as i32);
                    if invalid(vpos, hpos, width, height) {
                        return lowest;
                    }

                    let current_lower = heatmap[i][j] < heatmap[vpos as usize][hpos as usize];
                    current_lower && lowest
                });

            if is_lowest {
                low_points.push((i, j));
                sum += heatmap[i][j] + 1;
            }
        }
    }

    println!("{}", sum);

    low_points
}

fn find_basin(heatmap: &mut Vec<Vec<u32>>, vpos: i32, hpos: i32) -> u32 {
    let height = heatmap.len();
    let width = heatmap[0].len();

    if invalid(vpos, hpos, width, height) {
        return 0;
    }

    if heatmap[vpos as usize][hpos as usize] == 9 {
        return 0;
    }

    // marked
    heatmap[vpos as usize][hpos as usize] = 9;

    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    1 + directions.iter()
        .fold(0, |c, (dv, dh)| c + find_basin(heatmap, dv + vpos, dh + hpos))
}

fn part2(heatmap: &mut Vec<Vec<u32>>, low_points: Vec<(usize, usize)>) -> u32 {
    let mut basins: Vec<u32> = low_points.iter()
        .map(|(a, b)| (*a as i32, *b as i32))
        .map(|(a, b)| find_basin(heatmap, a, b))
        .collect();

    basins.sort();
    basins.iter()
        .rev()
        .take(3)
        .product()
}

fn main() {
    let parse_line = |line: &str| -> Vec<u32> {
        line.split("")
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
    };

    let mut heatmap: Vec<Vec<u32>> = include_str!("input").trim().lines()
        .map(parse_line)
        .collect();

    let low_points = part1(&heatmap);
    println!("{}", part2(&mut heatmap, low_points));
}
