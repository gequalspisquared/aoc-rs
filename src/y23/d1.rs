use std::fs;

pub fn run(file_path: &str) {
    let lines = fs::read_to_string(file_path).expect("Failed to read file");

    p1(&lines);
    p2(&lines);
}

fn parse_line_p1(line: &str) -> i32 {
    let mut c1 = '0';
    let mut c2 = '0';
    let mut found_first = false;
    for c in line.chars() {
        if c.is_numeric() {
            c2 = c;
            if found_first {
                continue;
            } else {
                c1 = c;
                found_first = true;
            }
        }
    }

    let s = format!("{}{}", c1, c2);
    return s.parse::<i32>().unwrap();
}

fn parse_line_p2(line: &str) -> i32 {
    const DIGITS: [&'static str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut c1 = '0';
    let mut c2 = '0';
    let mut min = usize::MAX;
    let mut max = 0;

    for (i, digit) in DIGITS.iter().enumerate() {
        if let Some(pos) = line.rfind(digit) {
            if pos >= max {
                max = pos;
                c2 = (i + 1).to_string().chars().next().unwrap();
            }
        }

        if let Some(pos) = line.find(digit) {
            if pos <= min {
                min = pos;
                c1 = (i + 1).to_string().chars().next().unwrap();
            }
        }
    }

    for i in 1..=9 {
        let num = i.to_string().chars().next().unwrap();
        if let Some(pos) = line.rfind(num) {
            if pos >= max {
                max = pos;
                c2 = num;
            }
        }

        if let Some(pos) = line.find(num) {
            if pos <= min {
                min = pos;
                c1 = num;
            }
        }
    }

    let s = format!("{}{}", c1, c2);
    return s.parse::<i32>().unwrap();
}

fn p1(lines: &str) {
    let mut sum = 0;
    for line in lines.lines() {
        sum += parse_line_p1(line);
    }

    println!("Sum: {sum}");
}

fn p2(lines: &str) {
    let mut sum = 0;
    for line in lines.lines() {
        sum += parse_line_p2(line);
    }

    println!("Sum: {sum}");
}
