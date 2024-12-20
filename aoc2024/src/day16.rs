use array2d::Array2D;
use std::collections::LinkedList;

pub fn run() {
    let input = include_str!("../input16.txt");

    let grid_lines = input.split("\n").collect::<Vec<&str>>();
    let mut grid = Array2D::filled_with(' ', grid_lines.len(), grid_lines[0].len());

    let mut start_pos = (0, 0);
    let mut end_pos;
    for i in 0..grid_lines.len() {
        let line = grid_lines[i];
        // println!("{}", line);
        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();
            grid[(i, j)] = c;
            if c == 'S' {
                start_pos = (i, j);
                println!("start_pos: {:?}", start_pos);
            } else if c == 'E' {
                end_pos = (i, j);
                println!("end_pos: ({:?})", end_pos);
            }
        }
    }

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rein_dir = 0;
    let mut positions = LinkedList::new();
    positions.push_front((start_pos, rein_dir, 0 /*score*/));

    let mut scores = Vec::new();

    loop {
        if positions.is_empty() {
            break;
        }
        let i = positions.pop_front().unwrap();
        println!("positions.pop(): {:?}", i);
        let pos = i.0;
        let dir_i = i.1;
        let score = i.2;

        if grid[pos] == 'E' {
            println!("End: score: {}", score);
            scores.push(score);
            continue;
        }
        else if grid[pos] == '.' {
            grid[pos] = '#';
        }

        let d = dirs[dir_i];

        let new_pos = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize);
        // println!("new_pos: {:?}", new_pos);

        if grid[new_pos] == '.' || grid[new_pos] == 'E' {
            positions.push_front((new_pos, dir_i, score + 1));
        }

        for j in [-1i32, 1i32] {
            let turn_dir_i = (dir_i as i32 + j + 4) as usize % 4;
            let turn_d = dirs[turn_dir_i];
            let turn_new_pos = (
                (pos.0 as i32 + turn_d.0) as usize,
                (pos.1 as i32 + turn_d.1) as usize,
            );
            if grid[turn_new_pos] != '#' {
                positions.push_back((pos, turn_dir_i, score + 1000));
            }
        }
    }

    scores.sort();
    println!("{:?}", scores);
}
