use std::fs;

pub fn run(file_path: &str) {
    let scratchcard_info = fs::read_to_string(file_path).expect("Failed to get input!");
    let mut scratchcards = Vec::new();
    for line in scratchcard_info.lines() {
        scratchcards.push(Scratchcard::new(line));
    }

    p1(&scratchcards);
    p2(&scratchcards);
}

fn p1(scratchcards: &Vec<Scratchcard>) {
    let mut sum = 0;
    for scratchcard in scratchcards {
        let score = scratchcard.compute_score();
        sum += score;
    }
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

struct Scratchcard {
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl Scratchcard {
    fn new(line: &str) -> Scratchcard {
        let (_, right) = line.split_once(':').unwrap();

        let (winning_numbers, owned_numbers) = right.split_once('|').unwrap();
        let winning_numbers: Vec<&str> = winning_numbers.split_whitespace().into_iter().collect();
        let winning_numbers: Vec<u32> = winning_numbers
            .iter()
            .map(|s| s.parse::<u32>().expect("Failed to parse winning number"))
            .collect();
        let owned_numbers: Vec<&str> = owned_numbers.split_whitespace().into_iter().collect();
        let owned_numbers: Vec<u32> = owned_numbers
            .iter()
            .map(|s| s.parse::<u32>().expect("Failed to parse owned number"))
            .collect();

        Scratchcard {
            winning_numbers,
            owned_numbers,
        }
    }

    fn compute_score(&self) -> u32 {
        let mut score = 0;
        for owned_number in &self.owned_numbers {
            if self.winning_numbers.contains(&owned_number) {
                match score {
                    0 => score += 1,
                    _ => score *= 2,
                };
            }
        }

        score
    }
}
