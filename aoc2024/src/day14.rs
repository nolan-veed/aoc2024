use array2d::Array2D;
use regex::Regex;

pub fn run() {
    let input = include_str!("../input14.txt");
    let test = false;
    let part2 = true;

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

    if !part2 {
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
    } else {
        let mut new_ps = ps.clone();
        for t in 0..1000000 {
            for i in 0..new_ps.len() {
                let p = new_ps[i];
                let v = vs[i];
                let mut new_p = (p.0 + v.0, p.1 + v.1);
                new_p = (new_p.0.rem_euclid(sz.0), new_p.1.rem_euclid(sz.1));
                new_ps[i] = new_p;
            }
            // println!("new_ps: {:?}", new_ps);

            println!("t: {}", t);
            print_ps(&new_ps);
        }
    }
}

fn print_ps(ps: &Vec<(i32, i32)>) {
    let mut arr = Array2D::filled_with(0, 103, 101);

    for p in ps {
        arr[(p.1 as usize, p.0 as usize)] += 1;
    }
    let mut print_it = false;
    for i in 0..arr.num_rows() {
        let mut count = 0;
        for j in 0..arr.num_columns() {
            if arr[(i, j)] == 0 {
                count = 0;
            } else {
                count += 1;
                if count > 10 {
                    print_it = true;
                    break;
                }
            }
        }
        if print_it {
            break;
        }
    }
    if print_it {
        for i in 0..arr.num_rows() {
            for j in 0..arr.num_columns() {
                print!("{}", arr[(i, j)]);
            }
            println!();
        }
    }
    if print_it {
        println!("found");
    }
}
