use std::collections::HashMap;
use std::fs;

type SueInfo = HashMap<String, u32>;

pub fn run(file_path: &str) {
    let sue_list = fs::read_to_string(file_path).expect("Failed to get input!");
    let mut known_info: SueInfo = HashMap::new();
    known_info.insert(String::from("children"), 3);
    known_info.insert(String::from("cats"), 7);
    known_info.insert(String::from("samoyeds"), 2);
    known_info.insert(String::from("pomeranians"), 3);
    known_info.insert(String::from("akitas"), 0);
    known_info.insert(String::from("vizslas"), 0);
    known_info.insert(String::from("goldfish"), 5);
    known_info.insert(String::from("trees"), 3);
    known_info.insert(String::from("cars"), 2);
    known_info.insert(String::from("perfumes"), 1);

    let sue_information = parse_sue_list(&sue_list);

    p1(&known_info, &sue_information);
    p2(&known_info, &sue_information);
}

fn p1(known_info: &SueInfo, sue_information: &Vec<SueInfo>) {
    for (i, sue) in sue_information.iter().enumerate() {
        if matches(&known_info, sue) {
            println!("Potential match: Sue {}", i + 1);
        }
    }
}

fn p2(known_info: &SueInfo, sue_information: &Vec<SueInfo>) {
    for (i, sue) in sue_information.iter().enumerate() {
        if matches_outdated(&known_info, sue) {
            println!(
                "Potential match with outdated retroencabulator: Sue {}",
                i + 1
            );
        }
    }
}

fn matches(known_list: &SueInfo, sue: &SueInfo) -> bool {
    for (compound, amount) in sue {
        if known_list[compound] != *amount {
            return false;
        }
    }

    true
}

fn matches_outdated(known_list: &SueInfo, sue: &SueInfo) -> bool {
    for (compound, amount) in sue {
        if compound == "cats" || compound == "trees" {
            if known_list[compound] >= *amount {
                return false;
            }
        } else if compound == "pomeranians" || compound == "goldfish" {
            if known_list[compound] <= *amount {
                return false;
            }
        } else if known_list[compound] != *amount {
            return false;
        }
    }

    true
}

fn parse_sue_list(sue_list: &str) -> Vec<SueInfo> {
    let mut sue_information = Vec::new();
    for sue in sue_list.lines() {
        let mut info: SueInfo = HashMap::new();

        let words: Vec<&str> = sue.split(" ").collect();
        let trait1 = parse_remove_last(words[2]);
        let quantity1 = parse_remove_last(words[3]).parse::<u32>().unwrap();
        let trait2 = parse_remove_last(words[4]);
        let quantity2 = parse_remove_last(words[5]).parse::<u32>().unwrap();
        let trait3 = parse_remove_last(words[6]);
        let quantity3 = words[7].parse::<u32>().unwrap();

        info.insert(trait1, quantity1);
        info.insert(trait2, quantity2);
        info.insert(trait3, quantity3);

        sue_information.push(info);
    }

    sue_information
}

fn parse_remove_last(word: &str) -> String {
    let mut word = String::from(word);
    word.pop();
    word
}
