use regex::Regex;

pub fn run() {
    let input = include_str!("../input14.txt");
    let test = false;
    let mut sz = (11, 7);
    if !test {
        sz = (101, 103);
    }

    let mut ps = Vec::new();
    let mut vs = Vec::new();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        println!("line: {}", line);
        for mat in re.captures_iter(line) {
            println!("{:?}", mat.get(1).unwrap().as_str());
            let n1 = mat.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let n2 = mat.get(2).unwrap().as_str().parse::<i32>().unwrap();
            ps.push((n1, n2));
            let n3 = mat.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let n4 = mat.get(4).unwrap().as_str().parse::<i32>().unwrap();
            vs.push((n3, n4));
        }
    }
    println!("ps: {:?}", ps);
    println!("vs: {:?}", vs);

    let mut new_ps = Vec::new();

    for i in 0..ps.len() {
        let p = ps[i];
        let v = vs[i];
        let mut new_p = (p.0 + 100 * v.0, p.1 + 100 * v.1);
        new_p = (new_p.0.rem_euclid(sz.0), new_p.1.rem_euclid(sz.1));
        new_ps.push(new_p);
    }
    println!("new_ps: {:?}", new_ps);

    let halfsz = (sz.0 / 2, sz.1 / 2);

    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;
    for p in new_ps {
        if p.0 < halfsz.0 {
            if p.1 < halfsz.1 {
                count1 += 1;
            } else if p.1 > halfsz.1 {
                count2 += 1;
            }
        } else if p.0 > halfsz.0 {
            if p.1 < halfsz.1 {
                count3 += 1;
            } else if p.1 > halfsz.1 {
                count4 += 1;
            }
        }
    }
    println!("prod: {}", count1 * count2 * count3 * count4);
}
