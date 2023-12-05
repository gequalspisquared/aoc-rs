extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    sequence::preceded,
    combinator::map_res,
    multi::separated_list0,
    IResult,
};
use std::fs;

pub fn run(file_path: &str) {
    let almanac_info = fs::read_to_string(file_path).expect("Failed to get input!");
    let seeds = preceded(tag("seeds: "), parse_u64_list)(&almanac_info).unwrap().1;
    let almanac = Almanac::new(&almanac_info);

    p1(&almanac, &seeds);
    p2(&almanac, &seeds);
}

fn p1(almanac: &Almanac, seeds: &Vec<u64>) {
    let lowest_location = almanac.find_lowest_location(seeds);
    println!("Lowest location: {lowest_location}");
}

fn p2(almanac: &Almanac, ranges: &Vec<u64>) {
    let mut min = u64::MAX;
    let mut seeds = Vec::new();
    let mut i = 0;
    while i < ranges.len() {
        let start = ranges[i];
        let range = ranges[i + 1];
        for seed in start..start+range {
            seeds.push(seed);
        }


        min = std::cmp::min(min, almanac.find_lowest_location(&seeds));
        seeds.clear();

        i += 2;
    }

    // for i in (0..ranges.len()).step_by(2) {
    //     println!("i: {i}");
    //     let seed_range = SeedRange::new(ranges[i], ranges[i + 1]);
    //
    //     min = std::cmp::min(min, almanac.find_lowest_location_range(&seed_range));
    // }
    // let seed_range = SeedRange::new(ranges[0], ranges[1]);
    //     min = std::cmp::min(min, almanac.find_lowest_location_range(&seed_range));

    println!("Lowest location: {min}");
}

#[derive(Clone, Debug)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    fn new(start: u64, length: u64) -> SeedRange {
        SeedRange { start, length }
    }
}

#[derive(Clone)]
struct Range {
    destination: u64,
    source: u64,
    range: u64,
}

impl Range {
    fn new(destination: u64, source: u64, range: u64) -> Range {
        Range { 
            destination,
            source,
            range,
        }
    }

    fn is_in_range(&self, source: u64) -> bool {
        self.source <= source && source < self.source + self.range
    }

    fn convert(&self, source: u64) -> Option<u64> {
        if self.is_in_range(source) {
            let diff = source - self.source;
            Some(self.destination + diff)
        } else {
            None
        }
    }

    fn convert_seed_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        // if seed_range.start + seed_range.length - 1 < self.source || seed_range.start > self.source + self.range - 1 {
        //     return vec![seed_range];
        // }
        
        let end = seed_range.start + seed_range.length - 1;

        // completely on left or right of self
        if seed_range.start >= self.source + self.range || end < self.source {
            return vec![seed_range.clone()];
        }
        
        // left is outside of range
        if !self.is_in_range(seed_range.start) && self.is_in_range(end) {
            let l_length = self.source - seed_range.start;
            let l_seed_range = SeedRange::new(seed_range.start, l_length);
            let r_length = end - self.source;
            let r_seed_range = SeedRange::new(self.convert(self.source).unwrap(), r_length);
            return vec![l_seed_range, r_seed_range];
        }

        // right is outside of range
        if self.is_in_range(seed_range.start) && !self.is_in_range(end) {
            let l_length = self.source + self.range - seed_range.start;
            let l_seed_range = SeedRange::new(self.convert(seed_range.start).unwrap(), l_length);
            let r_length = end - self.source + self.range;
            let r_seed_range = SeedRange::new(self.source + self.range, r_length);
            return vec![l_seed_range, r_seed_range];

        }

        // completely contained in range
        if self.is_in_range(seed_range.start) && self.is_in_range(end) {
            let seed_range = SeedRange::new(self.convert(seed_range.start).unwrap(), seed_range.length);
            return vec![seed_range] 
        }

        // larger than self
        if !self.is_in_range(seed_range.start) && !self.is_in_range(end) {
            let l_length = self.source - seed_range.start;
            let l_seed_range = SeedRange::new(seed_range.start, l_length);
            let m_seed_range = SeedRange::new(self.convert(self.source).unwrap(), self.range);
            let r_length = end - self.source + self.range;
            let r_seed_range = SeedRange::new(self.source + self.range, r_length);
            return vec![l_seed_range, m_seed_range, r_seed_range];
        }

        vec![]
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(ranges: &Vec<Range>) -> Map {
        Map { ranges: ranges.clone() }
    }

    fn convert(&self, source: u64) -> Option<u64> {
        for range in self.ranges.iter() {
            if let Some(converted) = range.convert(source) {
                return Some(converted);
            }
        }

        None
    }

    fn convert_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let mut ranges = Vec::new();
        for range in self.ranges.iter() {
            for seed_range in range.convert_seed_range(seed_range) {
                ranges.push(seed_range);
            }
        }

        ranges
    }
}

struct Almanac {
    maps: Vec<Map>,
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn parse_u64_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(multispace1, parse_u64)(input)
}

impl Almanac {
    fn new(almanac_info: &str) -> Almanac {
        let mut maps = Vec::new();
        let mut current_map = Vec::new();
        for line in almanac_info.lines() {
            if line.contains(char::is_alphabetic) || line.trim().is_empty() {
                if !current_map.is_empty() {
                    maps.push(Map::new(&current_map));
                    current_map.clear();
                }
                continue;
            } 
            let nums = parse_u64_list(line).unwrap().1;
            current_map.push(Range::new(nums[0], nums[1], nums[2]));
        }
        maps.push(Map::new(&current_map));

        Almanac { maps }
    }

    fn find_lowest_location(&self, seeds: &Vec<u64>) -> u64 {
        let mut min = u64::MAX;
        for seed in seeds.iter() {
            let mut current_val = *seed;
            for map in self.maps.iter() {
                if let Some(converted) = map.convert(current_val) {
                    current_val = converted;
                }
            }

            min = std::cmp::min(min, current_val);
        }

        min
    }

    fn find_lowest_location_range(&self, seed_range: &SeedRange) -> u64 {
        let mut ranges = vec![seed_range.clone()];
        for map in self.maps.iter() {
            let mut current_ranges = vec![];
            let size = ranges.len();
            for i in 0..size {
                let new_ranges = map.convert_range(&ranges[i]);
                new_ranges.iter().for_each(|r| current_ranges.push(r.clone()));
                // current_ranges.push(new_ranges);
            }
            ranges = current_ranges;
        }

        // println!("Final SeedRanges: {:#?}", ranges);

        let mut min = u64::MAX;
        for range in ranges {
            min = std::cmp::min(min, range.start);
        }

        min
    }
}
