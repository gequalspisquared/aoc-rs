pub fn run() {
    p1(36_000_000);
    p2(36_000_000);
}

fn p1(min_number: usize) {
    let min_number = min_number / 10;
    let mut houses: Vec<usize> = vec![0; min_number];
    let mut house_number = min_number;

    for elf in 1..min_number {
        let mut house = elf;
        while house < min_number {
            houses[house] += elf;
            if houses[house] >= min_number && house < house_number {
                house_number = house;
            }

            house += elf;
        }
    }

    println!("{house_number}");
}

fn p2(min_number: usize) {
    let min_number = min_number / 11;
    let mut houses: Vec<usize> = vec![0; min_number];
    let mut house_number = min_number;

    for elf in 1..min_number {
        let mut house = elf;
        let mut steps = 0;
        while house < min_number && steps < 50 {
            houses[house] += elf;
            if houses[house] >= min_number && house < house_number {
                house_number = house;
            }

            house += elf;
            steps += 1;
        }
    }

    println!("{house_number}");
}
