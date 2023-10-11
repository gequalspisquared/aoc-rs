use std::fs;

pub fn run(file_path: &str) {
    let instructions = fs::read_to_string(file_path).expect("Failed to open input!");

    p1(&instructions);
    p2(&instructions);
}

fn p1(instructions: &str) {
    let mut lights = Lights::new();

    for instruction in instructions.lines() {
        let instruction = Instruction::parse(&instruction);
        lights.execute1(&instruction);
    }

    println!("Number of lights on: {}", lights.num_lights_on());
}

fn p2(instructions: &str) {
    let mut lights = Lights::new();

    for instruction in instructions.lines() {
        let instruction = Instruction::parse(&instruction);
        lights.execute2(&instruction);
    }

    println!("Number of lights on: {}", lights.num_lights_on());
}

struct Lights {
    lights: Vec<i32>,
}

impl Lights {
    fn new() -> Lights {
        Lights {
            lights: vec![0i32; 1_000 * 1_000],
        }
    }

    fn get_index(coordinate_pair: CoordinatePair) -> usize {
        (coordinate_pair.0 * 1_000 + coordinate_pair.1) as usize
    }

    fn execute1(&mut self, instruction: &Instruction) {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                let index = Lights::get_index(CoordinatePair(x, y));

                match &instruction.command {
                    Command::TurnOn => self.lights[index] = 1,
                    Command::TurnOff => self.lights[index] = 0,
                    Command::Toggle => {
                        if self.lights[index] == 0 {
                            self.lights[index] = 1;
                        } else {
                            self.lights[index] = 0;
                        }
                    }
                };
            }
        }
    }

    fn execute2(&mut self, instruction: &Instruction) {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                let index = Lights::get_index(CoordinatePair(x, y));

                match &instruction.command {
                    Command::TurnOn => self.lights[index] += 1,
                    Command::TurnOff => {
                        let current = self.lights[index];
                        self.lights[index] = if current == 0 { 0 } else { current - 1 };
                    }
                    Command::Toggle => self.lights[index] += 2,
                };
            }
        }
    }

    fn num_lights_on(&self) -> i32 {
        self.lights.iter().sum()
    }
}

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct CoordinatePair(u32, u32);

impl CoordinatePair {
    fn parse(pair: &str) -> CoordinatePair {
        let pair: Vec<&str> = pair.split(',').collect();
        let x = pair[0].parse::<u32>().unwrap();
        let y = pair[1].parse::<u32>().unwrap();

        CoordinatePair(x, y)
    }
}

struct Instruction {
    command: Command,
    from: CoordinatePair,
    to: CoordinatePair,
}

impl Instruction {
    fn parse(instruction: &str) -> Instruction {
        let instruction: Vec<&str> = instruction.split(' ').collect();

        let coordinate_index: usize;
        let command = match instruction[0] {
            "turn" => {
                coordinate_index = 2;
                match instruction[1] {
                    "on" => Command::TurnOn,
                    _ => Command::TurnOff,
                }
            }
            _ => {
                coordinate_index = 1;
                Command::Toggle
            }
        };

        let from_coordinate = instruction[coordinate_index];
        let to_coordinate = instruction[coordinate_index + 2];

        let from = CoordinatePair::parse(&from_coordinate);
        let to = CoordinatePair::parse(&to_coordinate);

        Instruction { command, from, to }
    }
}
