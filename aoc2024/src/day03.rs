pub fn day03() {
    let mut input = include_str!("../input03.txt");
    let part1 = false;

    let mut total = 0;

    if part1 {
        loop {
            let pos1 = input.find("mul(");
            if pos1 == None {
                break;
            }
            println!("pos1={}", pos1.unwrap());

            input = &input[(pos1.unwrap() + "mul(".len())..input.len()];

            let pos2 = input.find(")");
            if pos2 != None {
                let product = extract_and_mul(&input[0..pos2.unwrap()]);
                if product > 0 {
                    total += product;
                }
            }
        }
    } else {
        let mut enabled = true;

        loop {
            if enabled {
                let pos_dont = input.find("don't()");
                let pos1 = input.find("mul(");
                if pos1 == None {
                    break;
                }
                println!("pos1={}", pos1.unwrap());

                if pos_dont != None {
                    println!("pos_dont={}", pos_dont.unwrap());
                    if pos_dont.unwrap() < pos1.unwrap() {
                        enabled = false;
                        println!("disabled");
                        input = &input[(pos_dont.unwrap() + "don't()".len())..input.len()];
                        continue;
                    }
                }

                input = &input[(pos1.unwrap() + "mul(".len())..input.len()];

                let pos2 = input.find(")");
                if pos2 != None {
                    let product = extract_and_mul(&input[0..pos2.unwrap()]);
                    if product > 0 {
                        total += product;
                    }
                }
            } else {
                let pos_do = input.find("do()");
                if pos_do != None {
                    println!("pos_do={}", pos_do.unwrap());
                    enabled = true;
                    println!("disabled");
                    input = &input[(pos_do.unwrap() + "do()".len())..input.len()];
                }
                else {
                    break;
                }
            }
        }
    }
    println!("Total: {}", total);
}

fn extract_and_mul(input: &str) -> i32 {
    println!("input={}", input);

    let nums = input.split(",");

    if nums.clone().count() == 2 {
        let mut count = 0;
        let mut product = 1;
        for num in nums {
            println!("num={}", num);
            let parsed_num = num.parse::<i32>();
            if parsed_num.is_ok() {
                product *= parsed_num.unwrap();
                count += 1;
            }
        }
        if count == 2 {
            return product;
        }
    }

    return -1;
}
