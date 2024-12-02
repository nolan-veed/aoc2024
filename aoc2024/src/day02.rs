pub fn day02() {
    println!("Hello, world!");

    let input = include_str!("../input02.txt");

    let part2 = true;

    let mut safe_levels_count = 0;
    for line in input.lines() {
        let mut levels = Vec::new();

        let parts = line.split_whitespace().map(|s| s.parse::<i32>());

        for part in parts {
            levels.push(part.unwrap());
        }

        let mut is_safe = is_level_safe(&levels, part2);

        if is_safe {
            safe_levels_count += 1;
        } else if part2 {
            levels.reverse();
            is_safe = is_level_safe(&levels, part2);
            if is_safe {
                safe_levels_count += 1;
            }
        }
        println!("safe_levels_count: {}", safe_levels_count);
    }
}

pub fn is_level_safe(levels: &Vec<i32>, ignore_one_level: bool) -> bool {
    let mut is_safe = true;
    let mut one_level_ignored = false;

    let mut iter = levels.iter();
    let mut previous = iter.next().unwrap();
    let mut previous_diff = 0;
    for level in iter {
        let current = level;
        println!("previous: {}, current: {}", previous, current);

        let diff = current - previous;
        if diff.abs() < 1 || diff.abs() > 3 {
            is_safe = false;
        }
        if previous_diff != 0 && diff * previous_diff < 0 {
            is_safe = false;
        }

        if !is_safe {
            if ignore_one_level {
                if !one_level_ignored {
                    one_level_ignored = true;
                    is_safe = true;
                    continue;
                }
            }
        }
        if !is_safe {
            break;
        }
        previous = current;
        previous_diff = diff;
    }
    return is_safe;
}
