mod day01;
mod day02;
mod day03;

fn main() {
    let day = 3;
    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        _ => println!("Not implemented yet"),
    }
}
