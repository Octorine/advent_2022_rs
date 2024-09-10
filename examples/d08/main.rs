fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let grid: Vec<Vec<u32>> = std::fs::read_to_string(puzzle_file)
        .unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut left_view: Vec<Vec<u32>> = vec![vec!(0; grid[0].len()); grid.len()];
    let mut right_view = left_view.clone();
    let mut top_view = left_view.clone();
    let width = grid[0].len();
    let height = grid.len();
    let mut bottom_view = left_view.clone();
    for j in 0..height {
        for i in 0..width {
            left_view[j][i] = if i == 0 {
                grid[j][i]
            } else {
                left_view[j][i - 1].max(grid[j][i])
            };
            right_view[j][width - i - 1] = if i == 0 {
                grid[j][width - i - 1]
            } else {
                right_view[j][width - i].max(grid[j][width - i - 1])
            };
            top_view[j][i] = if j == 0 {
                grid[j][i]
            } else {
                top_view[j - 1][i].max(grid[j][i])
            };
            bottom_view[height - j - 1][i] = if j == 0 {
                grid[height - j - 1][i]
            } else {
                bottom_view[height - j][i].max(grid[height - j - 1][i])
            };
        }
    }

    let mut count = 0;
    for (j, row) in grid.clone().into_iter().enumerate() {
        for (i, tree) in row.into_iter().enumerate() {
            if i == 0
                || j == 0
                || i == width - 1
                || j == height - 1
                || tree > left_view[j][i - 1]
                || tree > right_view[j][i + 1]
                || tree > top_view[j - 1][i]
                || tree > bottom_view[j + 1][i]
            {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);

    let mut views: Vec<Vec<u32>> = vec![vec!(0; width); height];
    for j in 0..height {
        for i in 0..width {
            let mut left_view = 0;
            if i > 0 {
                let mut x = i - 1;
                while x > 0 && grid[j][x] < grid[j][i] {
                    left_view += 1;
                    x -= 1
                }
                left_view += 1
            }

            let mut right_view = 0;
            if i < width - 1 {
                let mut x = i + 1;
                while x < width - 1 && grid[j][x] < grid[j][i] {
                    right_view += 1;
                    x += 1
                }
                right_view += 1
            }

            let mut top_view = 0;
            if j > 0 {
                let mut y = j - 1;
                while y > 0 && grid[y][i] < grid[j][i] {
                    top_view += 1;
                    y -= 1
                }
                top_view += 1
            }

            let mut bottom_view = 0;
            if j < height - 1 {
                let mut y = j + 1;
                while y < height - 1 && grid[y][i] < grid[j][i] {
                    bottom_view += 1;
                    y += 1
                }
                bottom_view += 1
            }
            views[j][i] = left_view * right_view * top_view * bottom_view;
        }
    }
    println!(
        "Part 2: {}",
        views
            .into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
    )
}
