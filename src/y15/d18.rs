use std::fs;

pub fn run(file_path: &str) {
    let initial_configuration = fs::read_to_string(file_path)
        .expect("Failed to get input!");

    let lights = Lights::new(&initial_configuration);
    p1(&lights, 100);
    p2(&lights, 100);
}

fn p1(lights: &Lights, num_steps: usize) {
    let mut lights = lights.clone();

    for _ in 0..num_steps {
        lights.step();
    }
    println!("After {num_steps} steps, there are {} lights on", lights.count());
}

fn p2(lights: &Lights, num_steps: usize) {
    let mut lights = lights.clone();

    for _ in 0..num_steps {
        lights.step_corners_on();
    }
    println!("After {num_steps} corner on steps, there are {} lights on", lights.count());
}

#[derive(Clone)]
struct Lights {
    lights: Vec<i32>,
    size: usize,
}

impl Lights {
    fn new(initial_configuration: &str) -> Lights {
        let size = initial_configuration.lines().count();
        let mut lights = vec![0i32; size * size];

        for (y, row) in initial_configuration.lines().rev().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col == '#' {
                    lights[x + (size - y - 1)*size] = 1;
                }
            }
        }

        Lights { lights, size }
    }

    fn step(&mut self) {
        let lights = self.clone();
        
        for (idx, light) in lights.lights.iter().enumerate() {
            let (x, y) = (idx / self.size, idx % self.size);
            let num_neighbors = lights.get_neighbor_count(x as i32, y as i32);
            if *light > 0 && num_neighbors != 2 && num_neighbors != 3 {
                self.lights[idx] = 0;
            } else if *light == 0 && num_neighbors == 3 {
                self.lights[idx] = 1;
            }
        }
    }

    fn step_corners_on(&mut self) {
        self.set_corners_on();
        let lights = self.clone();
        
        for (idx, light) in lights.lights.iter().enumerate() {
            let (x, y) = (idx / self.size, idx % self.size);
            let num_neighbors = lights.get_neighbor_count(x as i32, y as i32);
            if *light > 0 && num_neighbors != 2 && num_neighbors != 3 {
                self.lights[idx] = 0;
            } else if *light == 0 && num_neighbors == 3 {
                self.lights[idx] = 1;
            }
        }
        self.set_corners_on();
    }

    fn set_corners_on(&mut self) {
        self.lights[0] = 1;
        self.lights[self.size - 1] = 1;
        self.lights[(self.size - 1) * self.size] = 1;
        self.lights[(self.size - 1) * self.size + (self.size - 1)] = 1;
    }

    fn get_neighbor_count(&self, x: i32, y: i32) -> i32 {
        let dirs: Vec<(i32, i32)> = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        let mut count = 0;

        for (dx, dy) in dirs {
            if x + dx >= self.size as i32 || x + dx < 0 ||
               y + dy >= self.size as i32 || y + dy < 0 {
                continue;
            }

            count += self.lights[(x + dx) as usize * self.size + (y + dy) as usize];
        }

        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        for x in 0..self.size {
            for y in 0..self.size {
                print!("{}", self.lights[x*6 + y]);
            }
            println!("");
        }
    }

    fn count(&self) -> i32 {
        self.lights.iter().sum()
    }
}
