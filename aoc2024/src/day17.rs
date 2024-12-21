pub fn run() {
    let input = include_str!("../input17.txt");

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let reg_lines = parts[0].split("\n").collect::<Vec<&str>>();

    let mut reg_a = reg_lines[0]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_b = reg_lines[1]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_c = reg_lines[2]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("{} {} {}", reg_a, reg_b, reg_c);

    let prog_lines = parts[1].split("\n").collect::<Vec<&str>>();

    let prog_line = prog_lines[0].split(" ").collect::<Vec<&str>>();

    println!("{:?}", prog_line[1]);
    let prog = prog_line[1]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{:?}", prog);

    let mut ip = 0;

    let mut output = Vec::new();
    loop {
        if ip >= prog.len() {
            println!("done");
            break;
        }
        let opcode = prog[ip];

        if opcode == 0 {
            let operand = prog[ip + 1];
            let power = get_combo(operand, reg_a, reg_b, reg_c);
            reg_a = reg_a / 2usize.pow(power as u32);
            ip += 2;
        } else if opcode == 1 {
            let operand = prog[ip + 1];
            let v = reg_b ^ operand;
            reg_b = v;
            ip += 2;
        } else if opcode == 2 {
            let operand = prog[ip + 1];
            let v = get_combo(operand, reg_a, reg_b, reg_c);
            reg_b = v % 8;
            ip += 2;
        } else if opcode == 3 {
            if reg_a == 0 {
                ip += 2;
            } else {
                let operand = prog[ip + 1];
                ip = operand;
            }
        } else if opcode == 4 {
            let v = reg_b ^ reg_c;
            reg_b = v;
            ip += 2;
        } else if opcode == 5 {
            let operand = prog[ip + 1];
            let v = get_combo(operand, reg_a, reg_b, reg_c);
            let out = v % 8;
            println!("out: {}", out);
            output.push(out);
            ip += 2;
        } else if opcode == 6 {
            let operand = prog[ip + 1];
            let power = get_combo(operand, reg_a, reg_b, reg_c);
            reg_b = reg_a / 2usize.pow(power as u32);
            ip += 2;
        } else if opcode == 7 {
            let operand = prog[ip + 1];
            let power = get_combo(operand, reg_a, reg_b, reg_c);
            reg_c = reg_a / 2usize.pow(power as u32);
            ip += 2;
        }
    }

    let output_str: String = output
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", output_str);
}

fn get_combo(operand: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    match operand {
        0..4 => operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("invalid combo operand"),
    }
}
