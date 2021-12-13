fn invalid(vpos: i32, hpos: i32, width: usize, height: usize) -> bool {
    if vpos < 0 || vpos >= height as i32 {
        return true;
    }

    if hpos < 0 || hpos >= width as i32 {
        return true;
    }
    false
}

fn part1(heatmap: Vec<Vec<u32>>) -> Vec<(usize, usize)> {
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

fn main() {
    let parse_line = |line: &str| -> Vec<u32> {
        line.split("")
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
    };

    let heatmap: Vec<Vec<u32>> = include_str!("input").trim().lines()
        .map(parse_line)
        .collect();



    let low_points = part1(heatmap);
}
