use std::fs;

pub fn run(file_path: &str) {
    let strings = fs::read_to_string(file_path).expect("Failed to open input!");

    p1(&strings);
    p2(&strings);
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    }
}

fn p1(strings: &str) {
    let mut num_nice_strings = 0;

    for string in strings.lines() {
        if is_string_nice_1(string) {
            num_nice_strings += 1;
        }
    }

    println!("Number of nice strings: {num_nice_strings}");
}

fn p2(strings: &str) {
    let mut num_nice_strings = 0;

    for string in strings.lines() {
        if is_string_nice_2(string) {
            num_nice_strings += 1;
        }
    }

    println!("Number of nice strings: {num_nice_strings}");
}

fn is_string_nice_1(string: &str) -> bool {
    let forbidden_strings = vec!["ab", "cd", "pq", "xy"];

    let mut num_vowels = 0;
    let mut appeared_twice = false;
    let mut contains_forbidden_string = false;

    let mut is_first = true;
    let mut c1: char;
    let mut c2 = 'a';

    for c in string.chars() {
        if is_vowel(c) {
            num_vowels += 1;
        }

        c1 = c2;
        c2 = c;

        if is_first {
            is_first = false;
        } else if c1 == c2 {
            appeared_twice = true;
        }
    }

    for forbidden_substring in forbidden_strings {
        if string.contains(forbidden_substring) {
            contains_forbidden_string = true;
            break;
        }
    }

    num_vowels >= 3 && appeared_twice && !contains_forbidden_string
}

fn is_string_nice_2(string: &str) -> bool {
    let mut contains_pair = false;
    let mut split_duplicate = false;

    let mut c1: char;
    let mut c2 = 'a';
    let mut c3 = 'a';
    
    for (i, c) in string.chars().enumerate() {
        c1 = c2;
        c2 = c3;
        c3 = c;

        if i > 0 {
            let substr: String = [c2, c3].iter().collect();
            if string.matches(&substr[..]).count() > 1 {
                contains_pair = true;
            }
        }

        if i > 1 && c1 == c3 {
            split_duplicate = true;
        }
    }

    contains_pair && split_duplicate
}
