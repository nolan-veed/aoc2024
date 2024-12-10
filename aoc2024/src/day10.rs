use array2d::Array2D;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input10.txt");

    let max_i = input.lines().count() - 1;
    let max_j = input.lines().clone().next().unwrap().len() - 1;

    let mut grid = Array2D::filled_with(0, max_i + 1, max_j + 1);

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            grid[(i, j)] = c.to_string().parse().unwrap();
            j += 1;
        }
        i += 1;
    }

    let mut count = 0;
    let mut ends: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..max_i + 1 {
        for j in 0..max_j + 1 {
            if grid[(i, j)] == 0 {
                find9(&grid, i, j, max_i, max_j, 0, &mut ends);
                println!("trailhead: {:?}, ends.len(): {}", (i, j), ends.len());
                count += ends.len();
                ends.clear();
            }
        }
    }

    println!("{}", count);
}

fn find9(
    grid: &Array2D<i32>,
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
    level: usize,
    mut ends: &mut HashSet<(usize, usize)>,
) -> u32 {
    let n = grid[(i, j)];
    println!("level: {} i: {}, j: {}, n: {}", level, i, j, n);

    if n == 9 {
        ends.insert((i, j));
        return 1;
    }
    let mut count = 0;
    let n_plus1 = n + 1;
    if i > 0 && grid[(i - 1, j)] == n_plus1 {
        count += find9(grid, i - 1, j, max_i, max_j, level + 1, &mut ends);
    }
    if i < max_i && grid[(i + 1, j)] == n_plus1 {
        count += find9(grid, i + 1, j, max_i, max_j, level + 1, &mut ends);
    }
    if j > 0 && grid[(i, j - 1)] == n_plus1 {
        count += find9(grid, i, j - 1, max_i, max_j, level + 1, &mut ends);
    }
    if j < max_j && grid[(i, j + 1)] == n_plus1 {
        count += find9(grid, i, j + 1, max_i, max_j, level + 1, &mut ends);
    }

    count
}
