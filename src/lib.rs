mod y15;
mod y23;

pub fn run(year: &str, day: &str) {
    match year {
        "2015" => y15::run(&day),
        "2023" => y23::run(&day),
        _ => panic!("Could not determine year!"),
    };
}
