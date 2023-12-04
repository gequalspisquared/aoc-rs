use std::fs;

pub fn run(file_path: &str) {
    let scratchcard_info = fs::read_to_string(file_path).expect("Failed to get input!");
    let mut scratchcards = Vec::new();
    scratchcard_info
        .lines()
        .for_each(|line| scratchcards.push(Scratchcard::new(line)));

    p1(&scratchcards);
    p2(&scratchcards);
}

fn p1(scratchcards: &Vec<Scratchcard>) {
    let sum = scratchcards
        .iter()
        .fold(0, |acc, s| acc + s.compute_score());
    println!("Sum: {sum}");
}

fn p2(scratchcards: &Vec<Scratchcard>) {
    let mut instances = vec![1; scratchcards.len()];
    for (i, scratchcard) in scratchcards.iter().enumerate() {
        let score = scratchcard.compute_score();
        let num_winning_numbers = match score {
            0 => 0,
            _ => score.trailing_zeros() + 1,
        };

        for j in i + 1..=i + num_winning_numbers as usize {
            instances[j] += instances[i];
        }
    }

    let num_scratchcards: u32 = instances.iter().sum();

    println!("Total scratchcards: {num_scratchcards}");
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    let numbers: Vec<&str> = numbers.split_whitespace().into_iter().collect();
    numbers
        .iter()
        .map(|s| s.parse::<u32>().expect("Failed to parse number"))
        .collect()
}

struct Scratchcard {
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl Scratchcard {
    fn new(line: &str) -> Scratchcard {
        let (_, right) = line.split_once(':').unwrap();

        let (winning_numbers, owned_numbers) = right.split_once('|').unwrap();
        let winning_numbers = parse_numbers(winning_numbers);
        let owned_numbers = parse_numbers(owned_numbers);

        Scratchcard {
            winning_numbers,
            owned_numbers,
        }
    }

    fn compute_score(&self) -> u32 {
        self.owned_numbers.iter().fold(0, |acc, n| {
            if self.winning_numbers.contains(n) {
                match acc {
                    0 => 1,
                    _ => acc * 2,
                }
            } else {
                acc
            }
        })
    }
}
