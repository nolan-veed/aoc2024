use array2d::Array2D;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input12.txt");
    let part2 = true;

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
            let mut price = area_perimeter.0;
            if !part2 {
                price *= area_perimeter.1;
            } else {
                price *= area_perimeter.2;
            }
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
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize, usize) {
    if visited.contains(&(i, j)) {
        return (0, 0, 0);
    }
    let max_i = grid.row_len() - 1;
    let max_j = grid.column_len() - 1;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut stack = Vec::new();
    stack.push((i, j));

    let mut area = 0;
    let mut perimeter = 0;

    let mut perimeter_t = Vec::new();
    let mut perimeter_r = Vec::new();
    let mut perimeter_b = Vec::new();
    let mut perimeter_l = Vec::new();

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

        for di in 0..dirs.len() {
            let d = dirs[di];
            let next_pos = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);

            if next_pos.0 < 0 {
                perimeter += 1;
                perimeter_t.push(pos);
                continue;
            }
            if next_pos.0 > max_i as i32 {
                perimeter += 1;
                perimeter_b.push(pos);
                continue;
            }

            if next_pos.1 < 0 {
                perimeter += 1;
                perimeter_l.push(pos);
                continue;
            }

            if next_pos.1 > max_j as i32 {
                perimeter += 1;
                perimeter_r.push(pos);
                continue;
            }

            if grid[(next_pos.0 as usize, next_pos.1 as usize)] != c {
                perimeter += 1;
                match di {
                    0 => perimeter_t.push(pos),
                    1 => perimeter_b.push(pos),
                    2 => perimeter_l.push(pos),
                    3 => perimeter_r.push(pos),
                    _ => println!(),
                }
                continue;
            }

            if visited.contains(&(next_pos.0 as usize, next_pos.1 as usize)) {
                continue;
            }

            stack.push((next_pos.0 as usize, next_pos.1 as usize));
        }
    }

    perimeter_t.sort();
    println!("perimeter_t: {:?}", perimeter_t);

    perimeter_b.sort();
    println!("perimeter_b: {:?}", perimeter_b);

    perimeter_l.sort_by(|a, b| {
        let _a = (a.1, a.0);
        let _b = (b.1, b.0);
        _a.cmp(&_b)
    });
    println!("perimeter_l: {:?}", perimeter_l);

    perimeter_r.sort_by(|a, b| {
        let _a = (a.1, a.0);
        let _b = (b.1, b.0);
        _a.cmp(&_b)
    });
    println!("perimeter_r: {:?}", perimeter_r);

    let sides_t = get_h_sides(perimeter_t);
    println!("sides_t: {}", sides_t);
    let sides_b = get_h_sides(perimeter_b);
    println!("sides_b: {}", sides_b);
    let sides_l = get_v_sides(perimeter_l);
    println!("sides_l: {}", sides_l);
    let sides_r = get_v_sides(perimeter_r);
    println!("sides_r: {}", sides_r);

    let sides = sides_t + sides_b + sides_l + sides_r;
    println!("sides: {}", sides);

    (area, perimeter, sides)
}

fn get_h_sides(perimeter: Vec<(usize, usize)>) -> usize {
    let mut sides = 1;
    if perimeter.len() > 1 {
        for i in 0..perimeter.len() - 1 {
            if perimeter[i].0 != perimeter[i + 1].0 {
                sides += 1;
            } else if perimeter[i].1 + 1 != perimeter[i + 1].1 {
                sides += 1;
            }
        }
    }
    sides as usize
}

fn get_v_sides(perimeter: Vec<(usize, usize)>) -> usize {
    let mut sides = 1;
    if perimeter.len() > 1 {
        for i in 0..perimeter.len() - 1 {
            if perimeter[i].1 != perimeter[i + 1].1 {
                sides += 1;
            } else if perimeter[i].0 + 1 != perimeter[i + 1].0 {
                sides += 1;
            }
        }
    }
    sides as usize
}
