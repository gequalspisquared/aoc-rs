use std::fs;

pub fn run(file_path: &str) {
    let schematic = fs::read_to_string(file_path).expect("Failed to read input!");

    let size = (schematic.len() as f32).sqrt() as usize;

    let mut mat: Mat = vec![vec![' '; size]; size];
    for (y, line) in schematic.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            mat[x][y] = c;
        }
    }

    let mut checked_mat: Mat = vec![vec![' '; size]; size];
    for y in 0..size {
        for x in 0..size {
            if mat[x][y].is_numeric() {
                if let Some(_) = check_surrounding(&mat, x, y) {
                    checked_mat[x][y] = mat[x][y]
                }
            }
        }
    }

    let mut filled: Mat = vec![vec![' '; size]; size];
    for y in 0..size {
        for x in 0..size {
            if checked_mat[x][y].is_numeric() {
                filled[x][y] = checked_mat[x][y];
                let mut x1 = x + 1;
                while x1 < size && mat[x1][y].is_numeric() {
                    filled[x1][y] = mat[x1][y];
                    x1 += 1;
                }
            }
        }
    }

    for y in (0..size).rev() {
        for x in (0..size).rev() {
            if filled[x][y].is_numeric() {
                filled[x][y] = filled[x][y];
                if x > 0 {
                    let mut x1 = x - 1;
                    while mat[x1][y].is_numeric() {
                        filled[x1][y] = mat[x1][y];
                        if x1 == 0 {
                            break;
                        }
                        x1 -= 1;
                    }
                }
            }
        }
    }

    let mut transpose: Mat = vec![vec![' '; size]; size];
    for y in 0..size {
        for x in 0..size {
            transpose[x][y] = filled[y][x];
        }
    }

    let mut nums = Vec::new();
    for line in transpose.iter() {
        let string: String = line.iter().collect();
        for num in string.split_whitespace() {
            nums.push(num.parse::<i32>().unwrap());
        }
    }

    println!("Sum: {}", nums.iter().sum::<i32>());

    // help me (part 2)
    let mut checked_mat: Mat = vec![vec![' '; size]; size];
    for y in 0..size {
        for x in 0..size {
            if mat[x][y].is_numeric() {
                if let Some(c) = check_surrounding(&mat, x, y) {
                    if c == '*' {
                        checked_mat[x][y] = mat[x][y]
                    }
                }
            } else if mat[x][y] == '*' && get_num_touching(&mat, x, y) == 2 {
                checked_mat[x][y] = '*';
            }
        }
    }

    for y in 0..size {
        for x in 0..size {
            if checked_mat[x][y].is_numeric() {
                if let Some(c) = check_surrounding(&checked_mat, x, y) {
                    if c == '*' {
                        checked_mat[x][y] = mat[x][y]
                    }
                } else {
                    checked_mat[x][y] = ' ';
                }
            } else if mat[x][y] == '*' && get_num_touching(&mat, x, y) == 2 {
                checked_mat[x][y] = '*';
            }
        }
    }

    let mut filled: Mat = vec![vec![' '; size]; size];
    for y in 0..size {
        for x in 0..size {
            if checked_mat[x][y].is_numeric() {
                filled[x][y] = checked_mat[x][y];
                let mut x1 = x + 1;
                while x1 < size && mat[x1][y].is_numeric() {
                    filled[x1][y] = mat[x1][y];
                    x1 += 1;
                }
            } else if checked_mat[x][y] == '*' {
                filled[x][y] = '*';
            }
        }
    }

    for y in (0..size).rev() {
        for x in (0..size).rev() {
            if filled[x][y].is_numeric() {
                filled[x][y] = filled[x][y];
                if x > 0 {
                    let mut x1 = x - 1;
                    while mat[x1][y].is_numeric() {
                        filled[x1][y] = mat[x1][y];
                        if x1 == 0 {
                            break;
                        }
                        x1 -= 1;
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for y in 0..size {
        for x in 0..size {
            if filled[x][y] == '*' {
                sum += get_gear_ratio(&filled, x, y);
            }
        }
    }

    println!("Sum: {sum}");
}

type Mat = Vec<Vec<char>>;

fn check_surrounding(mat: &Mat, x: usize, y: usize) -> Option<char> {
    for dy in -1..=1 {
        for dx in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }

            let x1 = x as i32 + dx;
            let y1 = y as i32 + dy;

            if x1 >= 0 && x1 < mat.len() as i32 && y1 >= 0 && y1 < mat.len() as i32 {
                if let Some(c) = check(mat, x1 as usize, y1 as usize) {
                    return Some(c);
                }
            }
        }
    }

    None
}

fn check(mat: &Mat, x: usize, y: usize) -> Option<char> {
    let c = mat[x][y];
    if c != ' ' && !c.is_numeric() {
        return Some(c);
    }

    None
}

// x and y should be location of the gear ('*') symbol
fn get_num_touching(mat: &Mat, x: usize, y: usize) -> usize {
    let mut num_touching = 0;

    for dy in -1..=1 {
        let mut l = false;
        let mut m = false;
        let mut r = false;
        for dx in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }

            let x1 = x as i32 + dx;
            let y1 = y as i32 + dy;

            if x1 >= 0 && x1 < mat.len() as i32 && y1 >= 0 && y1 < mat.len() as i32 {
                if dy == 0 {
                    if mat[x1 as usize][y1 as usize].is_numeric() {
                        num_touching += 1;
                        continue;
                    }
                }

                if mat[x1 as usize][y1 as usize].is_numeric() {
                    match dx {
                        -1 => l = true,
                        0 => m = true,
                        1 => r = true,
                        _ => panic!("guh"),
                    };
                }
            }
        }

        if dy != 0 {
            if l && !m && r {
                num_touching += 2;
            } else if l || m || r {
                num_touching += 1;
            }
        }
    }

    num_touching
}

// x and y should be location of the gear ('*') symbol
fn get_gear_ratio(transpose: &Mat, x: usize, y: usize) -> usize {
    let mut product = 1;
    let mut num: Vec<char> = Vec::new();
    for dy in -1..=1 {
        let mut l = false;
        let mut m = false;
        let mut r = false;
        for dx in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }

            let x1 = x as i32 + dx;
            let y1 = y as i32 + dy;

            if x1 >= 0 && x1 < transpose.len() as i32 && y1 >= 0 && y1 < transpose.len() as i32 {
                if dy == 0 {
                    if transpose[x1 as usize][y1 as usize].is_numeric() {
                        let mut x2 = x1;
                        while x2 > 0 && transpose[(x2 - 1) as usize][y1 as usize].is_numeric() {
                            x2 -= 1;
                        }

                        num.push(transpose[x2 as usize][y1 as usize]);
                        while x2 < transpose.len() as i32 - 1
                            && transpose[(x2 + 1) as usize][y1 as usize].is_numeric()
                        {
                            x2 += 1;
                            num.push(transpose[x2 as usize][y1 as usize]);
                        }

                        let num1: String = num.iter().collect();
                        product *= num1.parse::<usize>().unwrap();
                        num.clear();

                        continue;
                    }
                }

                if transpose[x1 as usize][y1 as usize].is_numeric() {
                    match dx {
                        -1 => l = true,
                        0 => m = true,
                        1 => r = true,
                        _ => panic!("guh"),
                    };
                }
            }
        }

        if dy != 0 {
            let y1 = y as i32 + dy;

            if m {
                let mut x2 = x as i32;

                while x2 > 0 && transpose[(x2 - 1) as usize][y1 as usize].is_numeric() {
                    x2 -= 1;
                }

                num.push(transpose[x2 as usize][y1 as usize]);
                while x2 < transpose.len() as i32 - 1
                    && transpose[(x2 + 1) as usize][y1 as usize].is_numeric()
                {
                    x2 += 1;
                    num.push(transpose[x2 as usize][y1 as usize]);
                }

                let num1: String = num.iter().collect();
                product *= num1.parse::<usize>().unwrap();
                num.clear();
            } else {
                if l {
                    let mut x2 = x as i32 - 1;
                    while x2 > 0 && transpose[(x2 - 1) as usize][y1 as usize].is_numeric() {
                        x2 -= 1;
                    }

                    num.push(transpose[x2 as usize][y1 as usize]);
                    while x2 < transpose.len() as i32 - 1
                        && transpose[(x2 + 1) as usize][y1 as usize].is_numeric()
                    {
                        x2 += 1;
                        num.push(transpose[x2 as usize][y1 as usize]);
                    }

                    let num1: String = num.iter().collect();
                    product *= num1.parse::<usize>().unwrap();
                    num.clear();
                }
                if r {
                    let mut x2 = x as i32 + 1;
                    while x2 > 0 && transpose[(x2 - 1) as usize][y1 as usize].is_numeric() {
                        x2 -= 1;
                    }

                    num.push(transpose[x2 as usize][y1 as usize]);
                    while x2 < transpose.len() as i32 - 1
                        && transpose[(x2 + 1) as usize][y1 as usize].is_numeric()
                    {
                        x2 += 1;
                        num.push(transpose[x2 as usize][y1 as usize]);
                    }

                    let num1: String = num.iter().collect();
                    product *= num1.parse::<usize>().unwrap();
                    num.clear();
                }
            }
        }
    }

    product
}
