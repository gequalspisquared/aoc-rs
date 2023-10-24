use std::fs;
use std::collections::{HashMap, HashSet};

type ReplacementMap<'a> = HashMap<&'a str, Vec<&'a str>>;
type ReverseReplacementMap<'a> = HashMap<&'a str, &'a str>;

pub fn run(file_path: &str) {
    let replacements_and_molecule = fs::read_to_string(file_path)
        .expect("Failed to get input!");

    let (replacements, rev_replacements, molecule) = parse_input(&replacements_and_molecule);

    p1(&replacements, &molecule);
    p2(&rev_replacements, &molecule);
}

fn p1(replacements: &ReplacementMap, molecule: &str) {
    let original_molecule = String::from(molecule);
    let chars: Vec<char> = original_molecule.chars().collect();
    let mut seen: HashSet<String> = HashSet::new();

    let mut i = 0;
    while i < chars.len() {
        let substr;
        if i < chars.len() - 1 && chars[i + 1].is_lowercase() {
            substr = chars[i].to_string() + &chars[i+1].to_string();
        } else {
            substr = chars[i].to_string();
        }
        
        if replacements.contains_key(substr.as_str()) {
            for replacement in replacements[substr.as_str()].iter() {
                let mut new_string = original_molecule.clone();

                new_string.replace_range(i..i+substr.len(), replacement);
                seen.insert(new_string);
            }
        }

        i += 1;
    }

    println!("New unique molecules: {}", seen.len());
}

fn p2(rev_replacements: &ReverseReplacementMap, molecule: &str) {
    let mut original_molecule = String::from(molecule);
    let mut non_e = first_non_e(molecule);
    let mut steps = 0;

    while non_e > 0 { 
        let chars: Vec<char> = original_molecule.chars().collect();
        let mut i = non_e as i32;
        let mut smallest_idx = chars.len();
        let mut key = "";
        while i >= 0 {
            let substr = &original_molecule[i as usize..=non_e];
            if rev_replacements.contains_key(substr) {
                smallest_idx = i as usize;
                key = substr;
            }

            i -= 1;
        }
        
        if key == "" {
            non_e -= 1;
            continue;
        }

        original_molecule.replace_range(smallest_idx..=non_e, rev_replacements[key]);

        non_e = first_non_e(&original_molecule);

        steps += 1;
    }

    match original_molecule.as_str() {
        "e" => println!("Num steps: {}", steps),
        _ => println!("Num steps: {}", steps + 1),
    };
}

fn first_non_e(molecule: &str) -> usize {
    for (i, c) in molecule.chars().rev().enumerate() {
        if c != 'e' {
            return molecule.len() - i - 1;
        }
    }
    
    0
}

fn parse_input(replacements_and_molecule: &str) -> (ReplacementMap, ReverseReplacementMap, &str) {
    let mut replacements: ReplacementMap = HashMap::new();
    let mut rev_replacements: ReverseReplacementMap = HashMap::new();
    let mut molecule = "";
    for line in replacements_and_molecule.lines() {
        if line.is_empty() {
            continue;
        }
        if !line.contains("=>") {
            molecule = line;
            break;
        }

        let words: Vec<&str> = line.split(" ").collect();
        replacements.entry(words[0]).or_insert(Vec::new());
        replacements.get_mut(words[0]).unwrap().push(words[2]);
        rev_replacements.insert(words[2], words[0]);
    }

    (replacements, rev_replacements, molecule)
}
