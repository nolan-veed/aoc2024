pub fn run() {
    let input = include_str!("../input07.txt");
    let part2 = true;

    let mut total = 0i64;

    for line in input.lines() {
        let mut parts = line.split(&[':', ' ']).map(|s| s.parse::<i64>());
        println!("parts_count={}", parts.clone().count());

        let lhs = parts.next().unwrap().unwrap();
        println!("lhs={}", lhs);

        let mut nums = Vec::new();
        for part in parts {
            if part.is_ok() {
                let v = part.unwrap();
                nums.push(v);
            }
        }
        println!("nums={:?}", nums);

        let found = find_combo(nums[0], &nums[1..nums.len()], lhs, 0, part2);
        if found {
            total += lhs;
        } else {
            let found = find_combo(nums[0], &nums[1..nums.len()], lhs, 1, part2);
            if found {
                total += lhs;
            } else {
                if part2 {
                    let found = find_combo(nums[0], &nums[1..nums.len()], lhs, 2, part2);
                    if found {
                        total += lhs;
                    }
                }
            }
        }
    }

    println!("total={}", total);
}

fn find_combo(num: i64, nums: &[i64], lhs: i64, op: i8, part2: bool) -> bool {
    // println!("find_combo: {} {:?} {} {}", num, nums, lhs, op);
    if nums.len() == 0 {
        if num == lhs {
            return true;
        }
        return false;
    }
    let mut next_num = num;
    if op == 0 {
        next_num *= nums[0];
    } else if op == 1 {
        next_num += nums[0];
    } else if op == 2 {
        let mut s = next_num.to_string();
        s += nums[0].to_string().as_str();
        next_num = s.parse::<i64>().unwrap();
    }
    let found = find_combo(next_num, &nums[1..nums.len()], lhs, 0, part2);
    if found {
        return true;
    } else {
        let found = find_combo(next_num, &nums[1..nums.len()], lhs, 1, part2);
        if found {
            return true;
        } else {
            if part2 {
                let found = find_combo(next_num, &nums[1..nums.len()], lhs, 2, part2);
                if found {
                    return true;
                }
            }
        }
    }
    false
}
