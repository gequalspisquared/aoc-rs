use std::fs;

pub fn run(file_path: &str) {
    let list = fs::read_to_string(file_path).expect("Failed to read input!");
    
    p1(&list);
    p2(&list);
}

fn p1(list: &str) {
    let mut num = 0;
    for line in list.lines() {
        let chars: Vec<char> = line.chars().collect();
        num += chars.len();
        let mut i = 1;
        while i < chars.len() - 1 {
            if chars[i] == '\\' {
                if chars[i + 1] == 'x' {
                    i += 4;
                } else {
                    i += 2;
                }
                
                num -= 1;
                continue;
            }

            num -= 1;
            i += 1;
        }
    }

    println!("The total is: {num}");
}

fn p2(list: &str) {
    let mut num = 0;
    for line in list.lines() {
        let chars: Vec<char> = line.chars().collect();
        num += 4;
        let mut i = 1;
        while i < chars.len() - 1 {
            if chars[i] == '\\' {
                if chars[i + 1] == 'x' {
                    num += 1;
                    i += 4;
                } else {
                    num += 2;
                    i += 2;
                }
                
                continue;
            }

            i += 1;
        }
    }

    println!("The total is: {num}");
}
