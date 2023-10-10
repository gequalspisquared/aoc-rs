use std::fs;

pub fn run(file_path: &str) {
    let instructions = fs::read_to_string(file_path).expect("Failed to read file");

    p1(&instructions);
    p2(&instructions);
}

fn p1(instructions: &str) {
    let mut floor = 0;

    for c in instructions.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        };
    }

    println!("Floor: {floor}");
}

fn p2(instructions: &str) {
    let mut floor = 0;

    for (i, c) in instructions.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        };

        if floor < 0 {
            println!("Entered basement at position: {}", i + 1);
            return;
        }
    }

    println!("Did not enter basement");
}
