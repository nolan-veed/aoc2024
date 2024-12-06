use array2d::Array2D;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input06.txt");

    let num_lines = input.lines().count();
    let line_len = input.lines().clone().next().unwrap().len();

    let mut grid = Array2D::filled_with(' ', num_lines, line_len);

    let mut guard = 0;
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
    println!("guard={}, guard_i={}, guard_j={}", guard, guard_i, guard_j);

    let mut dir_mapping: HashMap<usize, (i32, i32)> = HashMap::new();
    dir_mapping.insert(0, (0, -1));
    dir_mapping.insert(1, (1, 0));
    dir_mapping.insert(2, (0, 1));
    dir_mapping.insert(3, (-1, 0));

    let max_i = line_len - 1;
    let max_j = num_lines - 1;

    loop {
        let d = dir_mapping.get(&guard).unwrap();
        let next_i: i32 = guard_i as i32 + d.1;
        let next_j: i32 = guard_j as i32 + d.0;

        if next_i < 0 || next_i > max_i as i32 {
            grid[(guard_i, guard_j)] = 'X';
            break;
        }
        if next_j < 0 || next_j > max_j as i32 {
            grid[(guard_i, guard_j)] = 'X';
            break;
        }

        let next_c = grid[(next_i as usize, next_j as usize)];
        if next_c == '#' {
            println!("turn");
            guard = (guard + 1) % 4
        } else {
            println!("step");
            grid[(guard_i, guard_j)] = 'X';
            guard_i = next_i as usize;
            guard_j = next_j as usize;
        }
        println!(
            "loop: guard={}, guard_i={}, guard_j={}",
            guard, guard_i, guard_j
        );
    }

    let mut count = 0;

    for i in 0..max_i + 1 {
        for j in 0..max_j + 1 {
            if grid[(i, j)] == 'X' {
                count += 1;
            }
        }
    }

    println!("count={}", count);
}
