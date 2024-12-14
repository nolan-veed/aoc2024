use regex::Regex;

pub fn run() {
    let input = include_str!("../input13.txt");
    let part2 = true;

    let mut total_cost = 0i64;

    let mut nums = Vec::new();
    for line in input.lines() {
        if line.contains("Button A:") {
            nums.clear();
        }
        let re = Regex::new(r"\d+").unwrap();
        for mat in re.find_iter(line) {
            let n = line[mat.start()..mat.end()].parse::<i32>().unwrap();
            nums.push(n);
        }
        if nums.len() == 6 {
            println!("nums: {:?}", nums);

            if !part2 {
                let cost =
                    find_min_cost((nums[0], nums[1]), (nums[2], nums[3]), (nums[4], nums[5]));
                println!("cost: {}", cost);
                total_cost += cost as i64;
            } else {
                let cost = solve_machine(
                    (nums[0] as i64, nums[1] as i64),
                    (nums[2] as i64, nums[3] as i64),
                    (nums[4] as i64, nums[5] as i64),
                    10000000000000,
                );
                println!("cost: {}", cost);
                total_cost += cost;
            }
            nums.clear();
        }
    }

    println!("total_cost: {}", total_cost);
}

// https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
fn solve_machine(ma: (i64, i64), mb: (i64, i64), mprize: (i64, i64), offset: i64) -> i64 {
    let prize = (mprize.0 + offset, mprize.1 + offset);
    let det = ma.0 * mb.1 - ma.1 * mb.0;
    let a = (prize.0 * mb.1 - prize.1 * mb.0) / det;
    let b = (ma.0 * prize.1 - ma.1 * prize.0) / det;
    if (ma.0 * a + mb.0 * b, ma.1 * a + mb.1 * b) == (prize.0, prize.1) {
        a * 3 + b
    } else {
        0
    }
}
fn find_min_cost(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> i32 {
    let mut min_cost = 0;
    for i in 0..100 {
        for j in 0..100 {
            let pos = (a.0 * i + b.0 * j, a.1 * i + b.1 * j);
            if pos == c {
                let cost = i * 3 + j;
                if min_cost <= 0 || cost < min_cost {
                    min_cost = cost;
                }
            }
        }
    }

    min_cost
}
