use std::collections::{HashMap, HashSet};

pub fn day05() {
    let input = include_str!("../input05.txt");
    let part2 = true;

    let mut total = 0;

    let mut after_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in input.lines() {
        if line.len() > 0 {
            let mut parts = line.split(&['|', ',']).map(|s| s.parse::<i32>());
            let parts_count = parts.clone().count();
            if parts_count == 2 {
                let first = parts.next().unwrap().unwrap();
                let second = parts.next().unwrap().unwrap();
                println!("parts: {:?} {:?}", first, second);

                if !after_map.contains_key(&first) {
                    after_map.insert(first, HashSet::new());
                }
                after_map.get_mut(&first).unwrap().insert(second);
            } else if parts_count > 2 {
                let mut pages = vec![];
                for part in parts {
                    let page = part.unwrap();
                    pages.push(page);
                }

                if is_valid(&pages, &after_map) {
                    let middle = pages[pages.len() / 2];
                    println!("valid: middle={}", middle);
                    if !part2 {
                        total += middle;
                    }
                } else {
                    println!("not valid");
                    if part2 {
                        fix_order(&mut pages, &after_map);
                        let middle = pages[pages.len() / 2];
                        println!("fixed valid: middle={}", middle);
                        total += middle;
                    }
                }
            }
        }
    }

    println!("{}", total);
}

fn fix_order(pages: &mut Vec<i32>, after_map: &HashMap<i32, HashSet<i32>>) {
    for i in 0..pages.len() - 1 {
        for j in i + 1..pages.len() {
            let pages_after = after_map.get(&pages[j]);
            if pages_after.is_some() {
                if pages_after.unwrap().contains(&pages[i]) {
                    let temp = pages[i];
                    pages[i] = pages[j];
                    pages[j] = temp;
                }
            }
        }
    }
}
fn is_valid(pages: &Vec<i32>, after_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut valid = true;
    for i in 0..pages.len() - 1 {
        for j in i + 1..pages.len() {
            let pages_after = after_map.get(&pages[j]);
            if pages_after.is_some() {
                if pages_after.unwrap().contains(&pages[i]) {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            break;
        }
    }

    return valid;
}
