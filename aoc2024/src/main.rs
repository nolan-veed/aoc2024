mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let day = 5;
    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        _ => println!("Not implemented yet"),
    }
}
