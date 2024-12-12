use array2d::Array2D;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input12.txt");

    let max_i = input.lines().count() - 1;
    let max_j = input.lines().clone().next().unwrap().len() - 1;

    let mut grid = Array2D::filled_with(' ', max_i + 1, max_j + 1);

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            grid[(i, j)] = c;
            j += 1;
        }
        i += 1;
    }

    let mut visited = HashSet::new();

    let mut total_price = 0;
    for i in 0..max_i + 1 {
        for j in 0..max_j + 1 {
            let area_perimeter = find_area_perimeter(&grid, i, j, &mut visited);
            let price = area_perimeter.0 * area_perimeter.1;
            if price > 0 {
                println!(
                    "i: {} j: {} area_perimeter: {:?} price: {}",
                    i, j, area_perimeter, price
                );
            }
            total_price += price;
        }
    }
    println!("total_price: {}", total_price);
}

fn find_area_perimeter(
    grid: &Array2D<char>,
    i: usize,
    j: usize,
    mut visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    if visited.contains(&(i, j)) {
        return (0, 0);
    }
    let max_i = grid.row_len() - 1;
    let max_j = grid.column_len() - 1;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut stack = Vec::new();
    stack.push((i, j));

    let mut area = 0;
    let mut perimeter = 0;

    loop {
        if stack.is_empty() {
            break;
        }
        let pos = stack.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert((pos.0, pos.1));
        area += 1;

        let c = grid[(pos.0, pos.1)];

        for d in dirs {
            let next_pos = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);

            if next_pos.0 < 0 || next_pos.0 > max_i as i32 {
                perimeter += 1;
                continue;
            }

            if next_pos.1 < 0 || next_pos.1 > max_j as i32 {
                perimeter += 1;
                continue;
            }

            if grid[(next_pos.0 as usize, next_pos.1 as usize)] != c {
                perimeter += 1;
                continue;
            }

            if visited.contains(&(next_pos.0 as usize, next_pos.1 as usize)) {
                continue;
            }

            stack.push((next_pos.0 as usize, next_pos.1 as usize));
        }
    }

    (area, perimeter)
}
