pub fn run() {
    let input = include_str!("../input09.txt");
    let part2 = true;

    let line = input.lines().next().unwrap();

    let mut blocks = Vec::new();
    let mut id = 0;
    let mut i = 0;
    for c in line.chars() {
        let d = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            for _j in 0..d {
                blocks.push(id);
            }

            id += 1;
        } else {
            for _j in 0..d {
                blocks.push(-1);
            }
        }
        i += 1;
    }
    // println!("blocks: {:?}", blocks);
    println!("blocks.len(): {}", blocks.len());

    if !part2 {
        let mut i1 = 0;
        let mut i2 = blocks.len() - 1;
        loop {
            if i1 >= blocks.len() {
                break;
            }
            if i2 == 0 {
                break;
            }
            if i1 >= i2 {
                break;
            }
            if blocks[i1] != -1 {
                i1 += 1;
                continue;
            }
            if blocks[i2] == -1 {
                i2 -= 1;
                continue;
            }
            blocks.swap(i1, i2);
        }
    } else {
        let mut i1: i32 = blocks.len() as i32 - 1;
        let mut count = 0;
        loop {
            if i1 < 1 {
                break;
            }
            if blocks[i1 as usize] == -1 {
                i1 -= 1;
                count = 0;
                continue;
            }
            count += 1;
            let i1prev = i1 - 1;
            if blocks[i1prev as usize] != blocks[i1 as usize] {
                for i2 in 0..i1 - count + 1 {
                    let mut found = true;
                    for i3 in i2..i2 + count {
                        if blocks[i3 as usize] != -1 {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        for count_i in 0..count {
                            blocks.swap((i1 + count_i) as usize, (i2 + count_i) as usize);
                        }
                        break;
                    }
                }
                count = 0;
            }
            i1 -= 1;
        }
    }
    // println!("blocks: {:?}", blocks);

    let mut checksum = 0i64;
    let mut i3 = 0;
    for b in blocks {
        if b != -1 {
            checksum += b * i3;
        }
        i3 += 1;
    }
    println!("checksum: {}", checksum);
}
