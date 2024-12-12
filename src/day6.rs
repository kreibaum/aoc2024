//! Wandering Guard Problem

use crate::utils::read_file;
use crate::utils::square_grid::SquareCharacterGrid;
use std::collections::HashSet;


pub fn main() {
    // Read information from day 6 file.
    let file = read_file("day6");
    let grid = SquareCharacterGrid::new(&file);

    let mut walls: HashSet<(i32, i32)> = HashSet::new();

    let mut guard_x: i32 = 0;
    let mut guard_y: i32 = 0;

    for (x, y, c) in grid.iter() {
        if c == '#' {
            walls.insert((x, y));
        } else if c == '^' {
            guard_x = x;
            guard_y = y;
        }
    }

    let (w, h) = grid.size();

    // Print the area.
    // print_area(&walls, w, h, guard_x, guard_y);

    let init_direction = Direction { dx: 0, dy: -1 };

    let (steps_taken, seen_positions, seen_statespace) =
        walk_throuh_lab(guard_x, guard_y, init_direction, &walls, w, h)
            .expect("Guard must walk off the board");

    println!("Size of the area: {}x{}", w, h);
    println!("Guard at: ({}, {})", guard_x, guard_y);
    println!("Steps taken: {}", steps_taken);
    println!("Seen positions: {}", seen_positions.len());
    println!("Seen statespace: {}", seen_statespace.len());

    // assert_eq!(seen_positions.len(), 5199);

    // Part 2
    //walls.insert((3, 6));
    //walk_throuh_lab(guard_x, guard_y, init_direction, &walls, w, h);

    let mut positions_with_a_loop = 0;
    /*for x in 0..w {
        for y in 0..h {
            let mut new_walls = walls.clone();
            new_walls.insert((x, y));
            let walk = walk_throuh_lab(guard_x, guard_y, init_direction, &new_walls, w, h);
            if walk.is_none() {
                positions_with_a_loop += 1;
            }
        }
    }*/
    for (x, y) in seen_positions.iter() {
        let mut new_walls = walls.clone();
        new_walls.insert((*x, *y));
        let walk = walk_throuh_lab(guard_x, guard_y, init_direction, &new_walls, w, h);
        if walk.is_none() {
            positions_with_a_loop += 1;
        }
    }
    println!("Positions with a loop: {}", positions_with_a_loop);
}


fn walk_throuh_lab(
    init_guard_x: i32,
    init_guard_y: i32,
    init_direction: Direction,
    walls: &HashSet<(i32, i32)>,
    w: i32,
    h: i32,
) -> Option<(i32, HashSet<(i32, i32)>, HashSet<(i32, i32, i32, i32)>)> {
    let mut guard_x = init_guard_x;
    let mut guard_y = init_guard_y;
    let mut direction = init_direction;

    let mut steps_taken = 0;
    let mut seen_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut seen_statespace: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    // Let the guard move, as long as she is on the board
    while 0 <= guard_x && guard_x < w && 0 <= guard_y && guard_y < h {
        // Guards sees the square she is on
        seen_positions.insert((guard_x, guard_y));
        let new_state = (guard_x, guard_y, direction.dx, direction.dy);
        if seen_statespace.contains(&new_state) {
            return None;
        }
        seen_statespace.insert(new_state);
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
    Some((steps_taken, seen_positions, seen_statespace))
}


#[derive(Copy, Clone)]
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
