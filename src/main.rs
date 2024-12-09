//! Shared utility functions
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

/// Read a file to a string.
pub fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(format!("input/{}", filename)).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    // Read information from day 6 file.
    let file = read_file("day6");
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut seen_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut guard_x: i32 = 0;
    let mut guard_y: i32 = 0;

    let mut w: i32 = 0;
    let mut h: i32 = 0;

    // Parse the file into a 2D array of booleans.
    for (y, line) in file.lines().enumerate() {
        h = (y + 1) as i32;
        for (x, c) in line.chars().enumerate() {
            w = (x + 1) as i32;
            if c == '#' {
                walls.insert((x as i32, y as i32));
            } else if c == '^' {
                guard_x = x as i32;
                guard_y = y as i32;
            }
        }
    }

    // Print the area.
    // print_area(&walls, w, h, guard_x, guard_y);

    let mut direction = Direction { dx: 0, dy: -1 };

    let mut steps_taken = 0;

    // Let the guard move, as long as she is on the board
    while 0 <= guard_x && guard_x < w && 0 <= guard_y && guard_y < h {
        // Guards sees the square she is on
        seen_positions.insert((guard_x, guard_y));
        // Check if there is a wall in front of the guard.
        let front = (guard_x + direction.dx, guard_y + direction.dy);
        let is_wall = walls.contains(&front);
        if is_wall {
            direction = direction.turn_right();
        } else {
            guard_x += direction.dx;
            guard_y += direction.dy;
            steps_taken += 1;
        }
    }

    println!("Size of the area: {}x{}", w, h);
    println!("Steps taken: {}", steps_taken);
    println!("Seen positions: {}", seen_positions.len());
}

struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        // We use an upside down coordinate system, this is why the rotation is inverted.
        Direction {
            dx: -self.dy,
            dy: self.dx,
        }
    }
}

fn print_area(area: &HashSet<(i32, i32)>, w: i32, h: i32, guard_x: i32, guard_y: i32) {
    for y in 0..h {
        for x in 0..w {
            if area.contains(&(x as i32, y as i32)) {
                print!("#");
            } else if x == guard_x && y == guard_y {
                print!("G");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

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
