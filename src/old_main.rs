//! Shared utility functions
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

fn day2() {
    // Read file for day 2, then parse it line by line.
    let mut parsed: Vec<Vec<i64>> = vec![];
    let file = read_file("day2");
    for line in file.lines() {
        let mut row: Vec<i64> = vec![];
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        parsed.push(row);
    }
    println!("{:?}", parsed);

    let mut safe_count = 0;
    for row in parsed.iter() {
        println!("Row: {:?}", row);
        let is_safe = is_strictly_ordered_max_n(row, 1) || is_strictly_ordered_max_n(row, -1);
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Safe count: {}", safe_count);

    assert_eq!(safe_count, 321);

    let mut safe_count = 0;
    for row in parsed.iter() {
        println!("Row: {:?}", row);
        let is_safe =
            is_ordered_after_removing_one(row, 1) || is_ordered_after_removing_one(row, -1);
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Safe count: {}", safe_count);

    assert_eq!(safe_count, 386);
}

const MAX_STEP: i64 = 3;

fn is_strictly_ordered_max_n(row: &[i64], flip: i64) -> bool {
    find_ordering_problem(row, flip).is_none()
}

fn find_ordering_problem(row: &[i64], flip: i64) -> Option<usize> {
    for i in 0..row.len() - 1 {
        let step = (row[i + 1] - row[i]) * flip;
        if step > MAX_STEP || step <= 0 {
            return Some(i);
        }
    }
    None
}

fn is_ordered_after_removing_one(row: &[i64], flip: i64) -> bool {
    let Some(defect) = find_ordering_problem(row, flip) else {
        return true;
    };
    // Try removing the problem element
    let mut new_row = row.to_vec();
    new_row.remove(defect);
    if is_strictly_ordered_max_n(&new_row, flip) {
        return true;
    } else if defect == row.len() - 2 {
        return true;
    }
    // We might also need to remove the element after the defect
    new_row = row.to_vec();
    new_row.remove(defect + 1);
    is_strictly_ordered_max_n(&new_row, flip)
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
        let left: i128 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right: i128 = caps.get(2).unwrap().as_str().parse().unwrap();

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
    let mut map: HashMap<i128, i128> = HashMap::new();
    for &x in right_list.iter() {
        *map.entry(x).or_insert(0) += 1;
    }

    // Find the similarity score
    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        similarity_score += map.get(&left_list[i]).cloned().unwrap_or(0) * left_list[i];
    }
    println!("Similarity score: {}", similarity_score);
}
