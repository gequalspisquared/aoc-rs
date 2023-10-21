use std::fs;

use serde_json;
use serde_json::Value;

// Solution was pretty much ripped from arturh85:
// https://github.com/arturh85/adventofcode-rust-2015/blob/master/src/day12.rs

pub fn run(file_path: &str) {
    let tmp = fs::read_to_string(file_path).expect("Failed to get input!");
    let data = tmp.as_bytes();

    let value: Value = serde_json::from_slice(data).unwrap();
    p1(&value);
    p2(&value);
}

fn p1(value: &Value) {
    let sum = count(value);
    println!("Sum: {sum}");
}

fn p2(value: &Value) {
    let sum = count_excluding_reds(value);
    println!("Sum without red objects: {sum}");
}

fn count(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(count).sum(),
        Value::Object(obj) => obj.values().map(count).sum(),
        _ => 0,
    }
}

fn count_excluding_reds(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(count_excluding_reds).sum(),
        Value::Object(obj) => {
            let mut sum = 0;
            for value in obj.values() {
                if let Value::String(value) = value {
                    if value == "red" {
                        return 0;
                    }
                }

                sum += count_excluding_reds(value);
            }

            sum
        }
        _ => 0,
    }
}
