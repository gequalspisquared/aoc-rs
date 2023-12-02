use std::fs;
use std::cmp::max;

pub fn run(file_path: &str) {
    let games = fs::read_to_string(file_path).expect("Failed to get input!");

    p1(&games);
    p2(&games);
}

fn p1(games: &str) {
    let mut sum = 0;
    for (id, game) in games.lines().enumerate() {
        let id = id + 1;
        let (r, g, b) = parse_game(game);

        if r <= 12 && g <= 13 && b <= 14 {
            sum += id;
        }
        // println!("r: {r}, g: {g}, b: {b}");
    }

    println!("Sum: {sum}");
}

fn p2(games: &str) {
    let mut sum = 0;
    for game in games.lines() {
        let (r, g, b) = parse_game(game);

        sum += r * g * b;
    }

    println!("Sum: {sum}");
}

fn parse_game(game: &str) -> (i32, i32, i32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let (_, rounds) = game.split_once(':').unwrap();
    for round in rounds.split(';') {
        for colors in round.split(',') {
            let mut iter = colors.split_whitespace();
            let num = iter.next().unwrap();
            let color = iter.next().unwrap();
            let num = num.parse::<i32>().unwrap();
            match color {
                "red" => r = max(r, num),
                "green" => g = max(g, num),
                "blue" => b = max(b, num),
                _ => panic!("Could not parse game!"),
            };
        }
    }

    (r, g, b)
}
