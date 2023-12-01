mod d1;

pub fn run(day: &str) {
    let file_path = String::from("inputs/2023/d") + day + ".txt";
    // let file_path = String::from("inputs/2023/d") + day + "_example.txt";

    match day {
        "1" => d1::run(&file_path),
        _ => panic!("Could not determine day!"),
    };
}
