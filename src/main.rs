//! Shared utility functions
use std::{fs::File, io::Read};
use std::collections::HashMap;

/// Read a file to a string.
pub fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(format!("input/{}", filename)).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    // Read file for day 2, then parse it line by line.
    let mut parsed : Vec<Vec<i64>> = vec![];
    let file = read_file("day2");
    for line in file.lines() {
        let mut row : Vec<i64> = vec![];
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        parsed.push(row);
    }
    println!("{:?}", parsed);

    let mut safe_count = 0;
    for row in parsed.iter() {
        println!("Row: {:?}", row);
        let is_safe = is_strictly_increasing_max_n(row, 3) || is_strictly_decreasing_max_n(row, 3);
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Safe count: {}", safe_count);
}

fn is_strictly_increasing_max_n(row: &[i64], max_step: i64) -> bool {
    for i in 0..row.len() - 1 {
        let step = row[i + 1] - row[i];
        if step > max_step || step <= 0 {
            return false;
        }
    }
    true
}

fn is_strictly_decreasing_max_n(row: &[i64], max_step: i64) -> bool {
    for i in 0..row.len() - 1 {
        let step = row[i] - row[i + 1];
        if step > max_step || step <= 0 {
            return false;
        }
    }
    true
}

fn day1() {
    let mut left_list = vec![];
    let mut right_list = vec![];

    // Load file "input/day1-test" line by line. Parse it with the regex "^(\d+) +(\d+)$".
    let file = read_file("aoc1-in2.txt");
    for line in file.lines() {
        use regex::Regex;
        let re = Regex::new(r"^(\d+) +(\d+)$").unwrap();

        let caps = re.captures(line).unwrap();
        let left : i128 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right : i128 = caps.get(2).unwrap().as_str().parse().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Print first 5 entries of each list
    println!("Left list: {:?}", &left_list[0..5]);
    println!("Right list: {:?}", &right_list[0..5]);

    // Find the total distance
    let mut total_distance = 0;
    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs();
    }
    println!("Total distance: {}", total_distance);
    // assert!(total_distance == 1222801);

    // Part 2: Similarity score
    // Figure out how often each number appears in the right list.
    let mut map : HashMap<i128, i128> = HashMap::new();
    for &x in right_list.iter() {
        *map.entry(x).or_insert(0) += 1;
    }

    // Find the similarity score
    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        similarity_score += map.get(&left_list[i]).cloned().unwrap_or( 0 ) * left_list[i];
    }
    println!("Similarity score: {}", similarity_score);
}
