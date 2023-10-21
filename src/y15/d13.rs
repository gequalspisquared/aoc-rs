use std::cmp;
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

type AdjacencyList = Vec<Vec<i32>>;

pub fn run(file_input: &str) {
    let happiness_rules = fs::read_to_string(file_input).expect("Failed to get input!");
    let adjacency_list = parse_rules(&happiness_rules);

    p1(&adjacency_list);
    p2(&adjacency_list);
}

fn p1(adjacency_list: &AdjacencyList) {
    let names: Vec<usize> = (0..adjacency_list.len()).into_iter().collect();
    let mut max = 0;

    for arrangement in names.iter().permutations(names.len()) {
        let happiness = compute_total_happiness(adjacency_list, &arrangement);
        max = cmp::max(max, happiness);
    }

    println!("The optimal arrangement yields {max} happiness units");
}

fn p2(adjacency_list: &AdjacencyList) {
    // insert yourself
    let mut adjacency_list = adjacency_list.clone();
    for list in adjacency_list.iter_mut() {
        list.push(0);
    }
    adjacency_list.push(vec![0; adjacency_list.len()]);
    println!("{:#?}", adjacency_list);

    let names: Vec<usize> = (0..adjacency_list.len()).into_iter().collect();
    let mut max = 0;

    for arrangement in names.iter().permutations(names.len()) {
        let happiness = compute_total_happiness(&adjacency_list, &arrangement);
        max = cmp::max(max, happiness);
    }

    println!("The optimal arrangement yields {max} happiness units");
}

fn compute_total_happiness(adjacency_list: &AdjacencyList, arrangement: &Vec<&usize>) -> i32 {
    let mut sum = 0;
    for i in 0..adjacency_list.len() {
        let curr_person = *arrangement[i];
        if i == 0 {
            sum += adjacency_list[curr_person][*arrangement[arrangement.len() - 1]];
            sum += adjacency_list[curr_person][*arrangement[i + 1]];
        } else if i == adjacency_list.len() - 1 {
            sum += adjacency_list[curr_person][*arrangement[i - 1]];
            sum += adjacency_list[curr_person][*arrangement[0]];
        } else {
            sum += adjacency_list[curr_person][*arrangement[i - 1]];
            sum += adjacency_list[curr_person][*arrangement[i + 1]];
        }
    }

    sum
}

fn parse_rules(rules: &str) -> AdjacencyList {
    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut adjacency_list: AdjacencyList = vec![Vec::new()];

    for rule in rules.lines() {
        let words: Vec<&str> = rule.split(" ").collect();
        let first = words[0].to_string();
        let scale = match words[2] {
            "gain" => 1,
            "lose" => -1,
            _ => {
                println!("Could not determine if gains or loses!");
                0
            }
        };
        let happiness = words[3].parse::<i32>().expect("Not a number!");
        let mut last = words[10].to_string(); // remove period
        last.pop();

        let idx = name_to_index.len();
        let first_idx = *name_to_index.entry(first.clone()).or_insert(idx);
        let idx = name_to_index.len();
        let last_idx = *name_to_index.entry(last.clone()).or_insert(idx);

        let max_idx = if first_idx < last_idx {
            last_idx
        } else {
            first_idx
        };
        if adjacency_list.len() <= max_idx {
            adjacency_list.resize(max_idx + 1, Vec::new());
        }
        if adjacency_list[first_idx].len() <= max_idx {
            adjacency_list[first_idx].resize(max_idx + 1, 0);
        }
        if adjacency_list[last_idx].len() <= max_idx {
            adjacency_list[last_idx].resize(max_idx + 1, 0);
        }

        adjacency_list[first_idx][last_idx] = scale * happiness;
    }

    adjacency_list
}
