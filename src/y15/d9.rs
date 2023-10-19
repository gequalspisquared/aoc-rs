use std::fs;
use std::cmp;
use std::collections::HashMap;

use itertools::Itertools;

type AdjacencyList = Vec<Vec<usize>>;

pub fn run(file_path: &str) {
    let distances = fs::read_to_string(file_path).expect("Failed to read input!");
    let adjacency_list = parse_distances(&distances);

    p1(&adjacency_list);
    p2(&adjacency_list);
}

fn p1(adjacency_list: &AdjacencyList) {
    let locations: Vec<usize> = (0..adjacency_list.len()).into_iter().collect();
    let mut min = usize::MAX;

    for route in locations.iter().permutations(locations.len()) {
        let dist = traverse_route(adjacency_list, &route);
        min = cmp::min(min, dist);
    }

    println!("min: {min}");
}

fn p2(adjacency_list: &AdjacencyList) {
    let locations: Vec<usize> = (0..adjacency_list.len()).into_iter().collect();
    let mut max = 0;

    for route in locations.iter().permutations(locations.len()) {
        let dist = traverse_route(adjacency_list, &route);
        max = cmp::max(max, dist);
    }

    println!("max: {max}");
}

fn traverse_route(adjacency_list: &AdjacencyList, route: &Vec<&usize>) -> usize {
    let mut dist = 0;
    for i in 0..adjacency_list.len()-1 {
        dist += adjacency_list[*route[i]][*route[i+1]];
    }

    dist
}

fn parse_distances(distances: &str) -> AdjacencyList {
    let mut location_to_index: HashMap<String, usize> = HashMap::new();
    let mut adjacency_list: AdjacencyList = vec![Vec::new()];
    for route in distances.lines() {
        // parse route
        let route: Vec<String> = route
            .split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|str| String::from(str))
            .collect();
        let from = route[0].clone();
        let to = route[2].clone();
        let dist = route[4].parse::<usize>().expect("Could not parse distance");

        // correlate a location to an index
        let idx = location_to_index.len();
        let from_idx = *location_to_index.entry(from.clone()).or_insert(idx);
        let idx = location_to_index.len();
        let to_idx = *location_to_index.entry(to.clone()).or_insert(idx);

        // grow adjacency_list if a new location is added
        let max_idx = if from_idx < to_idx { to_idx } else { from_idx };
        if adjacency_list.len() <= max_idx {
            adjacency_list.resize(max_idx + 1, Vec::new());
        }
        if adjacency_list[from_idx].len() <= max_idx {
            adjacency_list[from_idx].resize(max_idx + 1, 0);
        }
        if adjacency_list[to_idx].len() <= max_idx {
            adjacency_list[to_idx].resize(max_idx + 1, 0);
        }
        
        // insert
        adjacency_list[from_idx][to_idx] = dist;
        adjacency_list[to_idx][from_idx] = dist;
    }
    
    adjacency_list
}
