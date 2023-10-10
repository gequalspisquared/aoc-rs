use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn run(file_path: &str) {
    let directions = fs::read_to_string(file_path).expect("Could not read input file");

    p1(&directions);
    p2(&directions);
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn origin() -> Location {
        Location { x: 0, y: 0 }
    }

    fn process_direction(&mut self, direction: char) {
        match direction {
            '>' => self.x += 1,
            '^' => self.y += 1,
            '<' => self.x -= 1,
            'v' => self.y -= 1,
            _ => (),
        };
    }
}

fn p1(directions: &str) {
    let mut locations_visited = HashMap::new();
    let mut current_location = Location::origin();
    locations_visited.insert(current_location, 1);

    for direction in directions.chars() {
        current_location.process_direction(direction);
        let count = locations_visited.entry(current_location).or_insert(1);
        *count += 1;
    }

    println!("Santa visited {} unique houses.", locations_visited.len());
}

fn p2(directions: &str) {
    let mut locations_visited = HashMap::new();
    let mut santa_current_location = Location::origin();
    let mut robo_current_location = Location::origin();
    locations_visited.insert(santa_current_location, 2);

    for (i, direction) in directions.chars().enumerate() {
        let count: &mut i32;
        if i % 2 == 0 {
            // santa moves
            santa_current_location.process_direction(direction);
            count = locations_visited.entry(santa_current_location).or_insert(1);
        } else {
            // robo santa moves
            robo_current_location.process_direction(direction);
            count = locations_visited.entry(robo_current_location).or_insert(1);
        }
        *count += 1;
    }

    println!(
        "Santa and Robo Santa visited {} unique houses.",
        locations_visited.len()
    );
}
