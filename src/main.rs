use std::env;

use aoc_rs::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (year, day) = (&args[1], &args[2]);

    run(&year, &day);
}
