// Disclaimer: This code is just a rust implementation of Dmitry Sebakov's 
// solution at: https://dmatrix.dev/advent-of-code-year-2015-day-7/

use std::fs;

use std::collections::HashMap;

pub fn run(file_path: &str) {
    let instructions = fs::read_to_string(file_path).expect("Failed to get input!");

    let mut instructions_map = HashMap::new();

    for instruction in instructions.lines() {
        let instruction: Vec<&str> = instruction.split(' ').collect();
        let instruction: Vec<String> = instruction
            .into_iter()
            .map(|str| String::from(str))
            .collect();
        let var: String = instruction.last().clone().unwrap().to_string();
        instructions_map.insert(var, instruction);
    }

    p1(&instructions_map);
    p2(&instructions_map);
}

type Signal = u16;
type InstructionsMap = HashMap<String, Vec<String>>;

fn p1(instructions_map: &InstructionsMap) {
    let mut instructions_map = instructions_map.clone();

    println!("{}", process(&mut instructions_map, "a"));
}

fn p2(instructions_map: &InstructionsMap) {
    let mut instructions_map_1 = instructions_map.clone();
    let a_val = process(&mut instructions_map_1, "a");

    let mut instructions_map_2 = instructions_map.clone();
    *instructions_map_2.get_mut("b").unwrap() =
        vec![a_val.to_string(), String::from("->"), String::from("b")];

    println!("{}", process(&mut instructions_map_2, "a"));
}

fn process(instructions_map: &mut InstructionsMap, wire: &str) -> Signal {
    let value = compute_signal(instructions_map, wire);
    *instructions_map.get_mut(&String::from(wire)).unwrap() =
        vec![value.to_string(), String::from("->"), String::from(wire)];

    value
}

fn compute_signal(instructions_map: & mut InstructionsMap, wire: & str) -> Signal {
    let instruction = instructions_map.get(&String::from(wire)).unwrap().clone();
    match instruction[1].as_str() {
        "->" => assign(instructions_map, &instruction[0]),
        "AND" => and(instructions_map, &instruction[0], &instruction[2]),
        "OR" => or(instructions_map, &instruction[0], &instruction[2]),
        "LSHIFT" => lshift(instructions_map, &instruction[0], &instruction[2]),
        "RSHIFT" => rshift(instructions_map, &instruction[0], &instruction[2]),
        _ => {
            if instruction[0].as_str() == "NOT" {
                not(instructions_map, &instruction[1])
            } else {
                0
            }
        }
    }
}

fn compute_value(instructions_map: &mut InstructionsMap, x: &str) -> Signal {
    for c in x.chars() {
        if c.is_alphabetic() {
            return process(instructions_map, x);
        }
    }

    x.parse().unwrap()
}

fn assign(instructions_map: &mut InstructionsMap, x: &str) -> Signal {
    compute_value(instructions_map, x)
}

fn and(instructions_map: &mut InstructionsMap, x1: &str, x2: &str) -> Signal {
    compute_value(instructions_map, x1) & compute_value(instructions_map, x2)
}

fn or(instructions_map: &mut InstructionsMap, x1: &str, x2: &str) -> Signal {
    compute_value(instructions_map, x1) | compute_value(instructions_map, x2)
}

fn lshift(instructions_map: &mut InstructionsMap, x1: &str, x2: &str) -> Signal {
    compute_value(instructions_map, x1) << compute_value(instructions_map, x2)
}

fn rshift(instructions_map: &mut InstructionsMap, x1: &str, x2: &str) -> Signal {
    compute_value(instructions_map, x1) >> compute_value(instructions_map, x2)
}

fn not(instructions_map: &mut InstructionsMap, x: &str) -> Signal {
    !compute_value(instructions_map, x)
}
