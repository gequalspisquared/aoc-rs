mod y15;

pub fn run(year: &str, day: &str) {
    match year {
        "2015" => y15::run(&day),
        _ => panic!("Could not determine year!"),
    };
}
