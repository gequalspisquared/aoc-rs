// The backtrack function was ripped from a Combination Sum II problem
// on leetcode from nkorgik:
// https://leetcode.com/problems/combination-sum-ii/solutions/4110275/clean-and-easy-solution-backtracking-based-python/

use std::fs;

pub fn run(file_path: &str) {
    let nums = fs::read_to_string(file_path).expect("Could not get input!");
    let mut nums: Vec<u32> = nums
        .lines()
        .map(|str| str.parse::<u32>().unwrap())
        .collect();
    nums.sort();
    let target: u32 = 150;

    let mut combinations = Vec::new();
    let mut curr_combination: Vec<usize> = Vec::new();
    backtrack(
        &nums,
        target,
        0,
        0,
        &mut curr_combination,
        &mut combinations,
    );

    p1(&combinations);
    p2(&combinations);
}

fn p1(combinations: &Vec<Vec<usize>>) {
    println!("Num combinations: {}", combinations.len());
}

fn p2(combinations: &Vec<Vec<usize>>) {
    let mut min_bottles = usize::MAX;
    for combination in combinations {
        if combination.len() < min_bottles {
            min_bottles = combination.len();
        }
    }

    let mut num_combinations = 0;
    for combination in combinations {
        if combination.len() == min_bottles {
            num_combinations += 1;
        }
    }
    println!("Num combinations with minimum number of containers: {num_combinations}");
}

fn backtrack(
    nums: &Vec<u32>,
    target: u32,
    idx: usize,
    total: u32,
    curr_combination: &mut Vec<usize>,
    num_combinations: &mut Vec<Vec<usize>>,
) {
    // let mut idx = idx;
    if total == target {
        num_combinations.push(curr_combination.to_vec());
        return;
    }

    if idx >= nums.len() || total > target {
        return;
    }

    curr_combination.push(idx);
    backtrack(
        nums,
        target,
        idx + 1,
        total + nums[idx],
        curr_combination,
        num_combinations,
    );

    curr_combination.pop();

    backtrack(
        nums,
        target,
        idx + 1,
        total,
        curr_combination,
        num_combinations,
    );
}
