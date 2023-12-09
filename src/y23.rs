mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

pub fn run(day: &str) {
    let file_path = String::from("inputs/2023/d") + day + ".txt";
    // let file_path = String::from("inputs/2023/d") + day + "_example.txt";

    match day {
        "1" => d1::run(&file_path),
        "2" => d2::run(&file_path),
        "3" => d3::run(&file_path),
        "4" => d4::run(&file_path),
        "5" => d5::run(&file_path),
        "6" => d6::run(&file_path),
        "7" => d7::run(&file_path),
        _ => panic!("Could not determine day!"),
    };
}
