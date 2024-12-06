pub fn run() {
    let input = include_str!("../input04.txt");
    let part1 = false;

    let mut total = 0;

    if part1 {
        total += count(input);

        let input_transpose = transpose(input);
        total += count(input_transpose.as_str());

        let input_staircase_forward = staircase(input, true);
        let input_staircase_forward_transposed = transpose(&input_staircase_forward);
        total += count(&input_staircase_forward_transposed);

        let input_staircase_backward = staircase(input, false);
        let input_staircase_backward_transposed = transpose(&input_staircase_backward);
        total += count(&input_staircase_backward_transposed);
    } else {
        total = count_xmas(input);
    }

    println!("total: {}", total);
}

fn count_xmas(input: &str) -> usize {
    let mut total = 0;

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    let line_len = lines.first().unwrap().len();

    for i in 0..num_lines - 2 {
        for j in 0..line_len - 2 {
            let middle = &lines[i + 1][j + 1..j + 1 + 1];
            if middle == "A" {
                let tl = &lines[i][j..j + 1];
                let tr = &lines[i][j + 2..j + 2 + 1];
                let bl = &lines[i + 2][j..j + 1];
                let br = &lines[i + 2][j + 2..j + 2 + 1];

                let mut tl_br_valid = false;
                if tl == "M" && br == "S" {
                    tl_br_valid = true;
                } else if tl == "S" && br == "M" {
                    tl_br_valid = true;
                }

                let mut tr_bl_valid = false;
                if tr == "M" && bl == "S" {
                    tr_bl_valid = true;
                } else if tr == "S" && bl == "M" {
                    tr_bl_valid = true;
                }

                if tl_br_valid && tr_bl_valid {
                    total += 1;
                }
            }
        }
    }

    return total;
}

fn staircase(input: &str, forward: bool) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    // let line_len = lines.first().unwrap().len();

    let mut output = "".to_string();

    let mut count_begin = 0;

    for line in input.lines() {
        let mut new_line = "".to_string();
        let zeroes_begin = "0".repeat(count_begin);
        let zeroes_end = "0".repeat(num_lines - 1 - count_begin);
        if forward {
            new_line.push_str(&zeroes_begin);
        } else {
            new_line.push_str(&zeroes_end);
        }
        new_line.push_str(line);
        if forward {
            new_line.push_str(&zeroes_end);
        } else {
            new_line.push_str(&zeroes_begin);
        }
        new_line.push_str("\n");
        println!("{}", new_line);
        output.push_str(&new_line);
        count_begin += 1;
    }
    return output;
}
fn transpose(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    let line_len = lines.first().unwrap().len();

    let mut output = "".to_string();
    for i in 0..line_len {
        for j in 0..num_lines {
            let line = lines[j];
            output.push_str(&line[i..i + 1]);
        }
        output.push_str("\n");
    }
    return output;
}
fn count(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        print!("{}\n", line);
        let mut line_to_check = line;
        loop {
            let pos = line_to_check.find("XMAS");

            if pos == None {
                break;
            }
            total += 1;
            line_to_check = &line_to_check[pos.unwrap() + 4..line_to_check.len()];
        }
        line_to_check = line;
        loop {
            let pos = line_to_check.find("SAMX");

            if pos == None {
                break;
            }
            total += 1;
            line_to_check = &line_to_check[pos.unwrap() + 4..line_to_check.len()];
        }
    }
    println!("count total: {}", total);
    return total;
}
