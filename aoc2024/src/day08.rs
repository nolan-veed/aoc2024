use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../input08.txt");

    let max_i = input.lines().count() - 1;
    let max_j = input.lines().clone().next().unwrap().len() - 1;

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            if c != '.' {
                if !nodes.contains_key(&c) {
                    nodes.insert(c, Vec::new());
                }
                nodes.get_mut(&c).unwrap().push((i, j));
            }
            j += 1;
        }
        i += 1;
    }

    let mut positions = HashSet::new();
    let mut count = 0;

    for v in nodes.values() {
        for i in 0..v.len() - 1 {
            let pos_i = v[i];
            for j in i + 1..v.len() {
                let pos_j = v[j];

                let dist = (
                    pos_j.0 as i64 - pos_i.0 as i64,
                    pos_j.1 as i64 - pos_i.1 as i64,
                );

                let new_pos1 = (pos_i.0 as i64 - dist.0, pos_i.1 as i64 - dist.1);
                let new_pos2 = (pos_j.0 as i64 + dist.0, pos_j.1 as i64 + dist.1);

                if new_pos1.0 >= 0 && new_pos1.0 <= max_i as i64 {
                    if new_pos1.1 >= 0 && new_pos1.1 <= max_j as i64 {
                        println!("new_pos1: {:?}", new_pos1);
                        positions.insert(new_pos1);
                        count += 1;
                    }
                }
                if new_pos2.0 >= 0 && new_pos2.0 <= max_i as i64 {
                    if new_pos2.1 >= 0 && new_pos2.1 <= max_j as i64 {
                        println!("new_pos2: {:?}", new_pos2);
                        positions.insert(new_pos2);
                        count += 1;
                    }
                }
            }
        }
    }
    println!("count: {}, positions.len(): {}", count, positions.len());
}
