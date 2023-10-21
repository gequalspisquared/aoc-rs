pub fn run() {
    let input = "hepxcrrq";

    let new_password = p1(&input);
    p2(&new_password);
}

fn p1(input: &str) -> String {
    let mut password = String::from(input);
    while !password_passes(&password) {
        increment(&mut password);
    }

    println!("{input} -> {password}");
    password
}

fn p2(input: &str) {
    let mut password = String::from(input);
    increment(&mut password);
    while !password_passes(&password) {
        increment(&mut password);
    }

    println!("{input} -> {password}");
}

fn password_passes(current_password: &str) -> bool {
    let chars = current_password.chars().collect::<Vec<char>>();
    for c in &chars {
        if *c == 'i' || *c == 'o' || *c == 'l' {
            return false;
        }
    }

    let mut pairs = Vec::new();
    let mut i = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] && !pairs.contains(&chars[i]) {
            pairs.push(chars[i]);
            i += 1;
        }

        i += 1;
    }

    if pairs.len() < 2 {
        return false;
    }

    i = 0;
    while i < chars.len() - 2 {
        let d1 = chars[i + 1] as i32 - chars[i] as i32;
        if d1 == 1 {
            let d2 = chars[i + 2] as i32 - chars[i + 1] as i32;
            if d2 == 1 {
                return true;
            } else {
                i += 1;
            }
        }

        i += 1;
    }

    return false;
}

fn increment(current_password: &mut String) {
    let mut continue_incrementing = true;
    let tmp = current_password
        .chars()
        .rev()
        .map(|c| {
            if !continue_incrementing {
                return c;
            }

            if c == 'z' {
                return 'a';
            } else {
                continue_incrementing = false;
                return char::from_u32(c as u32 + 1).unwrap();
            }
        })
        .collect::<String>();

    *current_password = tmp.chars().rev().collect::<String>();
}
