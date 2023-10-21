use std::cmp;
use std::fs;

pub fn run(file_path: &str) {
    let reindeer_stats = fs::read_to_string(file_path).expect("Failed to read input!");

    let mut reindeer = Vec::new();
    for stats in reindeer_stats.lines() {
        reindeer.push(Reindeer::new(stats));
    }

    p1(&reindeer);
    p2(&reindeer);
}

fn p1(reindeer: &Vec<Reindeer>) {
    let mut max_dist = 0;
    const ELAPSED_TIME: u32 = 2503;

    for r in reindeer {
        let dist = r.calculate_distance_traveled(ELAPSED_TIME);
        max_dist = cmp::max(max_dist, dist);
    }

    println!("The maximum distance traveled was: {}", max_dist);
}

fn p2(reindeer: &Vec<Reindeer>) {
    let winning_score = compute_winning_reindeer_score(&reindeer);

    println!("The winning reindeer has a score of: {winning_score}");
}

fn compute_winning_reindeer_score(reindeer: &Vec<Reindeer>) -> u32 {
    let mut scores = Vec::new();
    let mut dists = Vec::new();
    for _ in 0..reindeer.len() {
        scores.push(0);
        dists.push(0);
    }

    const ELAPSED_TIME: u32 = 2503;

    for time in 1..=ELAPSED_TIME {
        let mut max_dist = 0;
        for (i, r) in reindeer.iter().enumerate() {
            dists[i] = r.calculate_distance_traveled(time);
            max_dist = cmp::max(max_dist, dists[i]);
        }

        for i in 0..reindeer.len() {
            if dists[i] == max_dist {
                scores[i] += 1;
            }
        }
    }

    *scores.iter().max().unwrap()
}

struct Reindeer {
    speed: u32,
    flight_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn new(stats: &str) -> Reindeer {
        let words: Vec<&str> = stats.split(" ").collect();
        let speed = words[3].parse::<u32>().unwrap();
        let flight_time = words[6].parse::<u32>().unwrap();
        let rest_time = words[13].parse::<u32>().unwrap();

        Reindeer {
            speed,
            flight_time,
            rest_time,
        }
    }

    fn calculate_distance_traveled(&self, curr_time: u32) -> u32 {
        let period = self.flight_time + self.rest_time;

        let mut dist = 0;

        let num_periods = curr_time / period;
        dist += num_periods * self.flight_time * self.speed;

        let remaining_time = curr_time % period;
        if remaining_time > self.flight_time {
            return dist + self.flight_time * self.speed;
        }

        dist + remaining_time * self.speed
    }
}
