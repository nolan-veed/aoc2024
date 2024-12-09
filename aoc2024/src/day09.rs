pub fn run() {
    let input = include_str!("../input09.txt");

    let line = input.lines().next().unwrap();

    let mut blocks = Vec::new();
    let mut id = 0;
    let mut i = 0;
    for c in line.chars() {
        let d = c.to_digit(10).unwrap();

        if i % 2 == 0 {
            for j in 0..d {
                blocks.push(id);
            }

            id += 1;
        } else {
            for j in 0..d {
                blocks.push(-1);
            }
        }
        i += 1;
    }
    // println!("blocks: {:?}", blocks);
    println!("blocks.len(): {}", blocks.len());

    let mut i1 = 0;
    let mut i2 = blocks.len() - 1;
    loop {
        if i1 >= blocks.len() {
            break;
        }
        if i2 < 0 {
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
    // println!("blocks: {:?}", blocks);

    let mut checksum = 0i64;
    let mut i3 = 0;
    for b in blocks {
        if b == -1 {
            break;
        }
        checksum += b * i3;
        i3 += 1;
    }
    println!("checksum: {}", checksum);
}
