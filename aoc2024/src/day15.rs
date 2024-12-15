use array2d::Array2D;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input15.txt");

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let grid_lines = parts[0].split("\n").collect::<Vec<&str>>();
    let mut grid = Array2D::filled_with(' ', grid_lines.len(), grid_lines[0].len());
    let mut robot_pos = (0, 0);
    for i in 0..grid_lines.len() {
        let line = grid_lines[i];
        println!("{}", line);
        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();
            grid[(i, j)] = c;
            if c == '@' {
                robot_pos = (i, j);
                println!("robot_pos: ({}, {})", i, j);
            }
        }
    }

    let move_lines = parts[1].split("\n").collect::<Vec<&str>>();
    let mut moves = Vec::new();
    for line in move_lines {
        for c in line.chars() {
            moves.push(c);
        }
    }
    println!("{:?}", moves);

    let mut dir = HashMap::<char, (i32, i32)>::new();
    dir.insert('^', (-1, 0));
    dir.insert('v', (1, 0));
    dir.insert('<', (0, -1));
    dir.insert('>', (0, 1));

    for m in moves {
        let d = dir[&m];
        let mut new_pos = ((robot_pos.0 as i32 + d.0) as usize, (robot_pos.1 as i32 + d.1) as usize);
        println!("new_pos: ({}, {})", new_pos.0, new_pos.1);
        if grid[new_pos] == '#' {
            continue;
        } else if grid[new_pos] == '.' {
            grid[robot_pos] = '.';
            grid[new_pos] = '@';
            robot_pos = new_pos;
        } else if grid[new_pos] == 'O' {
            let mut dot_pos = new_pos;
            loop {
                dot_pos = ((dot_pos.0 as i32 + d.0) as usize, (dot_pos.1 as i32 + d.1) as usize);
                if grid[dot_pos] == '.' {
                    grid[robot_pos] = '.';
                    grid[dot_pos] = 'O';
                    grid[new_pos] = '@';
                    robot_pos = new_pos;
                    break;
                } else if grid[dot_pos] == '#' {
                    break;
                }
            }
        }
        // print_grid(&grid);
    }

    let mut total = 0;
    for i in 0..grid.num_rows() {
        for j in 0..grid.num_columns() {
            if grid[(i,j)] == 'O' {
                let coord = 100*i+j;
                total += coord;
            }
        }
    }
    println!("total: {}", total);
}

fn print_grid(grid: &Array2D<char>) {
    for i in 0..grid.num_rows() {
        for j in 0..grid.num_columns() {
            print!("{}", grid[(i, j)]);
        }
        println!();
    }
}