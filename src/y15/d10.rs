pub fn run() {
    let input = "1321131112".chars().collect::<Vec<char>>();
    p1(&input);
    p2(&input);
}

fn p1(input: &Vec<char>) {
    let mut curr = input.clone();
    for _ in 0..40 {
        curr = take_turn(&curr);
    }

    println!("Len: {}", curr.len());
}

fn p2(input: &Vec<char>) {
    let mut curr = input.clone();
    for _ in 0..50 {
        curr = take_turn(&curr);
    }

    println!("Len: {}", curr.len());
}

fn take_turn(current_sequence: &Vec<char>) -> Vec<char> {
    let mut curr_len = 0;
    let mut curr_char = current_sequence[0];
    let mut result: Vec<char> = Vec::new();
    for c in current_sequence {
        if *c != curr_char {
            result.append(&mut curr_len.to_string().chars().collect::<Vec<char>>());
            result.push(curr_char);
            curr_len = 1;
            curr_char = *c;
            continue;
        } 

        curr_len += 1;
    }

    result.append(&mut curr_len.to_string().chars().collect::<Vec<char>>());
    result.push(curr_char);

    result
}
