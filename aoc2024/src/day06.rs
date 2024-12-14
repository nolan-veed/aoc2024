use array2d::Array2D;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../input06.txt");
    let part2 = true;

    let num_lines = input.lines().count();
    let line_len = input.lines().clone().next().unwrap().len();

    let mut grid = Array2D::filled_with(' ', num_lines, line_len);

    let guard_dir = 0;
    let mut guard_i = 0;
    let mut guard_j = 0;

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            grid[(i, j)] = c;
            if c == '^' {
                guard_i = i;
                guard_j = j;
            }
            j += 1;
        }
        i += 1;
    }

    let max_i = grid.row_len() - 1;
    let max_j = grid.column_len() - 1;

    let mut count = 0;

    if !part2 {
        traverse(&mut grid, guard_dir, guard_i, guard_j, false);

        for i in 0..max_i + 1 {
            for j in 0..max_j + 1 {
                if grid[(i, j)] == 'X' {
                    count += 1;
                }
            }
        }
    } else {
        let (_, mut path) = traverse(&mut grid, guard_dir, guard_i, guard_j, false);

        let mut path_blocks = HashSet::new();
        for p in path.iter() {
            path_blocks.insert((p.0, p.1));
        }
        println!("path_blocks.len()={:?}", path_blocks.len());

        path_blocks.remove(&(guard_i, guard_j));

        for p in path_blocks {
            let mut grid_copy = grid.clone();
            grid_copy[(p.0, p.1)] = '#';
            let (is_looping, _) = traverse(&mut grid_copy, guard_dir, guard_i, guard_j, true);
            if is_looping {
                count += 1;
            }
        }
    }
    println!("count={}", count);
}

fn traverse(
    grid: &mut Array2D<char>,
    mut guard_dir: usize,
    mut guard_i: usize,
    mut guard_j: usize,
    part2: bool,
) -> (bool, HashSet<(usize, usize, usize)>) {
    println!(
        "guard_dir={}, guard_i={}, guard_j={}",
        guard_dir, guard_i, guard_j
    );

    let mut dir_mapping: HashMap<usize, (i32, i32)> = HashMap::new();
    dir_mapping.insert(0, (0, -1));
    dir_mapping.insert(1, (1, 0));
    dir_mapping.insert(2, (0, 1));
    dir_mapping.insert(3, (-1, 0));

    let max_i = grid.row_len() - 1;
    let max_j = grid.column_len() - 1;

    let mut path = HashSet::new();

    loop {
        // println!(
        //     "loop: guard={}, guard_i={}, guard_j={}",
        //     guard_dir, guard_i, guard_j
        // );
        let d = dir_mapping.get(&guard_dir).unwrap();
        let next_i: i32 = guard_i as i32 + d.1;
        let next_j: i32 = guard_j as i32 + d.0;

        if next_i < 0 || next_i > max_i as i32 {
            path.insert((guard_i, guard_j, guard_dir));
            grid[(guard_i, guard_j)] = 'X';
            break;
        }
        if next_j < 0 || next_j > max_j as i32 {
            path.insert((guard_i, guard_j, guard_dir));
            grid[(guard_i, guard_j)] = 'X';
            break;
        }

        let next_c = grid[(next_i as usize, next_j as usize)];
        if next_c == '#' {
            // println!("turn");
            let p = (guard_i, guard_j, guard_dir);
            if part2 {
                if path.contains(&p) {
                    println!("loop detected");
                    return (true, path);
                }
            }
            path.insert(p);
            guard_dir = (guard_dir + 1) % 4
        } else {
            // println!("step");
            let p = (guard_i, guard_j, guard_dir);
            if part2 {
                if path.contains(&p) {
                    println!("loop detected");
                    return (true, path);
                }
            }
            path.insert(p);
            grid[(guard_i, guard_j)] = 'X';
            guard_i = next_i as usize;
            guard_j = next_j as usize;
        }
    }
    (false, path)
}
