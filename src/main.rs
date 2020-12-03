use structopt::StructOpt;
mod day01;
mod day02;
mod day03;

#[derive(StructOpt)]
struct Cli {
    day: u8,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        _ => println!("Unimplemented day: {}", args.day),
    }
}
