use std::fs;

pub fn run(file_path: &str) {
    let races = fs::read_to_string(file_path).expect("Failed to get input!");

    let mut iter = races.lines();
    let times = parse_line_p1(iter.next().unwrap());
    let distances = parse_line_p1(iter.next().unwrap());
    p1(&times, &distances);

    let mut iter = races.lines();
    let time = parse_line_p2(iter.next().unwrap());
    let distance = parse_line_p2(iter.next().unwrap());
    p2(time, distance);
}

fn p1(times: &Vec<u64>, distances: &Vec<u64>) {
    let product: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| compute_better_options(t, d))
        .collect::<Vec<u64>>()
        .iter()
        .product();

    println!("Product: {product}");
}

fn p2(time: u64, distance: u64) {
    let better = compute_better_options(time, distance);
    println!("Better: {better}");
}

fn get_distance_travelled(time_pressed: u64, total_time: u64) -> u64 {
    (total_time - time_pressed) * time_pressed
}

fn compute_better_options(total_time: u64, record_distance: u64) -> u64 {
    let first = (0..=total_time)
        .find(|t| get_distance_travelled(*t, total_time) > record_distance)
        .unwrap();

    let second = (0..=total_time)
        .rev()
        .find(|t| get_distance_travelled(*t, total_time) > record_distance)
        .unwrap();

    second - first + 1
}

fn parse_line_p1(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u64>().expect("Parse int error"))
        .collect()
}

fn parse_line_p2(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .expect("Parse int error")
}
