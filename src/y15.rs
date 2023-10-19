mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;
mod d11;

pub fn run(day: &str) {
    let file_path = String::from("inputs/2015/d") + day + ".txt";
    // let file_path = String::from("inputs/2015/d") + day + "_example.txt";

    match day {
        "1" => d1::run(&file_path),
        "2" => d2::run(&file_path),
        "3" => d3::run(&file_path),
        "4" => d4::run(),
        "5" => d5::run(&file_path),
        "6" => d6::run(&file_path),
        "7" => d7::run(&file_path),
        "8" => d8::run(&file_path),
        "9" => d9::run(&file_path),
        "10" => d10::run(),
        "11" => d11::run(),
        _ => panic!("Could not determine day!"),
    };
}
