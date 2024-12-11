pub fn run() {
    let input = include_str!("../input11.txt");

    let parts = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let mut stones: Vec<_> = parts.collect();

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
}
