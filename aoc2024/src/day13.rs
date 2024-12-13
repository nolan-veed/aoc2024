use regex::Regex;

pub fn run() {
    let input = include_str!("../input13.txt");

    let mut total_cost = 0;

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

            let cost = find_min_cost((nums[0], nums[1]), (nums[2], nums[3]), (nums[4], nums[5]));
            println!("cost: {}", cost);
            total_cost += cost;

            nums.clear();
        }
    }

    println!("total_cost: {}", total_cost);
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
