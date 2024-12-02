mod day01;
mod day02;

fn main() {
    let day = 2;
    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        _ => println!("Not implemented yet"),
    }
}
