use std::fs;
use std::cmp;

pub fn run(file_path: &str) {
    let dimensions = fs::read_to_string(file_path)
        .expect("Failed to read file");

    p1(&dimensions);
    p2(&dimensions);
}

struct Dimension {
    l: u32,
    w: u32,
    h: u32,
}

impl Dimension {
    fn new(dimension_line: &str) -> Dimension {
        let dimensions: Vec<&str> = dimension_line.split('x').collect();
        Dimension { 
            l: dimensions[0].parse().unwrap(),
            w: dimensions[1].parse().unwrap(),
            h: dimensions[2].parse().unwrap(),
        }
    }

    fn compute_surface_area(&self) -> u32 {
        let (l, w, h) = (self.l, self.w, self.h);

        let largest_dim = cmp::max(l, cmp::max(w, h));
        let smallest_side_area = l * w * h / largest_dim;

        2*l*w + 2*w*h + 2*h*l + smallest_side_area
    }

    fn compute_ribbon_length(&self) -> u32 {
        let (l, w, h) = (self.l, self.w, self.h);

        let largest_dim = cmp::max(l, cmp::max(w, h));
        let smallest_perimeter = 2*l + 2*w + 2*h - 2*largest_dim;

        smallest_perimeter + self.volume()
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

fn p1(dimensions: &str) {
    let mut total_paper = 0;
    for dimension_line in dimensions.lines() {
        let dimension = Dimension::new(dimension_line);
        
        total_paper += dimension.compute_surface_area();
    }

    println!("Total wrapping paper needed: {total_paper}");
}

fn p2(dimensions: &str) {
    let mut total_ribbon = 0;
    for dimension_line in dimensions.lines() {
        let dimension = Dimension::new(dimension_line);
        
        total_ribbon += dimension.compute_ribbon_length();
    }

    println!("Total ribbon needed: {total_ribbon}");
}
