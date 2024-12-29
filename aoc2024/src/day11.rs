use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../input11.txt");
    let part2 = true;

    let parts = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let mut stones: Vec<_> = parts.collect();

    if !part2 {
        for i in 0..25 {
            println!("i: {}", i);
            let mut stones2 = Vec::new();

            for s in stones.clone() {
                if s == 0 {
                    stones2.push(1);
                } else {
                    let ss = s.to_string();
                    let ss_len = ss.len();
                    if ss_len % 2 == 0 {
                        stones2.push(ss[0..ss_len / 2].parse().unwrap());
                        stones2.push(ss[ss_len / 2..ss_len].parse().unwrap());
                    } else {
                        stones2.push(s * 2024);
                    }
                }
            }
            stones = stones2;
        }
        //println!("stones: {:?}", stones);
        println!("stones count: {:?}", stones.len());
    } else {
        let mut count = 0i64;
        let mut cache = HashMap::new();
        for s in stones {
            count = count + count_stones(s, 75, &mut cache);
        }
        println!("count: {}", count);
    }
}

fn count_stones(stone: i64, blinks_left: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if blinks_left == 0 {
        return 1;
    }
    if cache.contains_key(&(stone, blinks_left)) {
        return cache[&(stone, blinks_left)];
    }
    if stone == 0 {
        let count = count_stones(1, blinks_left - 1, cache);
        cache.insert((0, blinks_left), count);
        return count;
    }
    let ss = stone.to_string();
    let ss_len = ss.len();
    if ss_len % 2 == 0 {
        let stone1 = ss[0..ss_len / 2].parse().unwrap();
        let stone2 = ss[ss_len / 2..ss_len].parse().unwrap();
        let count = count_stones(stone1, blinks_left - 1, cache)
            + count_stones(stone2, blinks_left - 1, cache);
        cache.insert((stone, blinks_left), count);
        return count;
    }
    let count = count_stones(stone * 2024, blinks_left - 1, cache);
    cache.insert((stone, blinks_left), count);
    count
}
