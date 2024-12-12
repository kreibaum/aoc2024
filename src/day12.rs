use crate::utils::read_file;
use crate::utils::square_grid::SquareCharacterGrid;
use std::collections::HashMap;

pub fn main() {
    run_day12_part1("day12-test-a");
    run_day12_part1("day12-test-b");
    run_day12_part1("day12");
}

fn run_day12_part1(filename: &str) {
    println!("Running day 12 part 1 with file {}", filename);
    let file = read_file(filename);
    let grid: SquareCharacterGrid<i16> = SquareCharacterGrid::new(&file);

    /*grid.print();*/

    let mut next_garden_index = 0;
    let mut gardens: HashMap<usize, Garden> = HashMap::new();
    // References "initial garden index -> final garden index"
    // But you may need to follow the chain to find the final garden index.
    let mut merges: HashMap<usize, usize> = HashMap::new();
    let mut original_garden_at_coordinate: HashMap<(i16, i16), usize> = HashMap::new();

    for (x, y, c) in grid.iter() {
        // Check the neighbors which have already been parsed
        let same_as_left = grid.get(x - 1, y) == Some(c);
        let same_as_top = grid.get(x, y - 1) == Some(c);
        if !same_as_left && !same_as_top {
            // The easiest case, just create a new garden.
            let garden = Garden {
                area: 1,
                circumference: 4,
            };
            gardens.insert(next_garden_index, garden);
            original_garden_at_coordinate.insert((x, y), next_garden_index);
            next_garden_index += 1;
        } else if same_as_left && !same_as_top {
            // Find the garden to the left and add the current coordinate to it.
            let garden_index =
                find_merged_garden_index(&original_garden_at_coordinate, &mut merges, x - 1, y);
            original_garden_at_coordinate.insert((x, y), garden_index);
            // Increase the area and circumference of the garden.
            let garden = gardens.get_mut(&garden_index).expect("garden not found");
            garden.area += 1;
            garden.circumference += 2;
        } else if !same_as_left && same_as_top {
            // Same as above, only different parent coordinates.
            let garden_index =
                find_merged_garden_index(&original_garden_at_coordinate, &mut merges, x, y - 1);
            original_garden_at_coordinate.insert((x, y), garden_index);
            // Increase the area and circumference of the garden.
            let garden = gardens.get_mut(&garden_index).expect("garden not found");
            garden.area += 1;
            garden.circumference += 2;
        } else {
            // This is the one complicated case, here we need to merge the top and the left garden.
            // Maybe; They might already belong to the same garden.
            let top_garden_index =
                find_merged_garden_index(&original_garden_at_coordinate, &mut merges, x, y - 1);
            let left_garden_index =
                find_merged_garden_index(&original_garden_at_coordinate, &mut merges, x - 1, y);
            original_garden_at_coordinate.insert((x, y), top_garden_index);
            if top_garden_index == left_garden_index {
                let garden = gardens
                    .get_mut(&top_garden_index)
                    .expect("garden not found");
                // No change to circumference, only the area increases.
                garden.area += 1;
            } else {
                // Merge gardens. Retain the top garden and merge the left garden into it.
                let left_garden = gardens
                    .remove(&left_garden_index)
                    .expect("garden not found");
                let top_garden = gardens
                    .get_mut(&top_garden_index)
                    .expect("garden not found");
                top_garden.area += left_garden.area + 1;
                // No additional circumference.
                top_garden.circumference += left_garden.circumference;
                merges.insert(left_garden_index, top_garden_index);
            }
        }
    }

    // The price of fence required for a region is found by multiplying that region's area by its perimeter
    let mut total_price = 0;
    let mut reduced_price = 0;

    // Count unbroken edges:
    let edge_count_horizontal = count_horizontal_unbroken_edges(
        &original_garden_at_coordinate,
        &mut merges,
        &gardens,
        grid.size(),
    );
    let edge_count_vertical = count_vertical_unbroken_edges(
        &original_garden_at_coordinate,
        &mut merges,
        &gardens,
        grid.size(),
    );

    // Print all gardens by size and area (and index)
    for (index, garden) in gardens.iter() {
        println!(
            "Garden {} has area {} and circumference {}",
            index, garden.area, garden.circumference
        );
        total_price += garden.area * garden.circumference;
        let edge_count =
            edge_count_horizontal.get(index).unwrap() + edge_count_vertical.get(index).unwrap();
        println!(
            "Unbroken edges: {} + {} = {}",
            edge_count_horizontal.get(index).unwrap(),
            edge_count_vertical.get(index).unwrap(),
            edge_count
        );
        reduced_price += garden.area * (edge_count);
    }
    println!("Total price: {}", total_price);
    println!("Reduced price: {}", reduced_price);
    println!();
}

// Finds the garden at a given coordinate
fn find_merged_garden_index(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    x: i16,
    y: i16,
) -> usize {
    let garden_index = original_garden_at_coordinate
        .get(&(x, y))
        .expect("No original garden defined!");
    find_garden_index(merges, *garden_index)
}

fn try_find_merged_garden_index(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    x: i16,
    y: i16,
) -> Option<usize> {
    let garden_index = original_garden_at_coordinate.get(&(x, y));
    match garden_index {
        Some(garden_index) => Some(find_garden_index(merges, *garden_index)),
        None => None,
    }
}

// Finds the root garden for a given garden index, compressing the chain along the way.
// See "Union Find Algorithm" for more information.
fn find_garden_index(merges: &mut HashMap<usize, usize>, garden_index: usize) -> usize {
    let Some(base_garden) = merges.get(&garden_index).cloned() else {
        return garden_index; // Garden is a root garden.
    };
    let root = find_garden_index(merges, base_garden);
    // Apply path compression
    if root != base_garden {
        merges.insert(garden_index, root);
    }
    root
}

struct Garden {
    area: usize,
    circumference: usize,
}

fn count_horizontal_unbroken_edges(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    gardens: &HashMap<usize, Garden>,
    (w, h): (i16, i16),
) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for (garden_index, _) in gardens {
        let mut total = 0;
        for scan_y in -1..h {
            total += count_horizontal_unbroken_edges_in_row(
                original_garden_at_coordinate,
                merges,
                *garden_index,
                scan_y,
                w,
            );
        }
        result.insert(*garden_index, total);
    }
    result
}

fn count_vertical_unbroken_edges(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    gardens: &HashMap<usize, Garden>,
    (w, h): (i16, i16),
) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for (garden_index, _) in gardens {
        let mut total = 0;
        for scan_x in -1..w {
            total += count_vertical_unbroken_edges_in_row(
                original_garden_at_coordinate,
                merges,
                *garden_index,
                scan_x,
                h,
            );
        }
        result.insert(*garden_index, total);
    }
    result
}

/// Counts all the edges that are between row scan_y and scan_y + 1
/// Works by reading a row and representing it as
///
///     00000111100100000
///     00001111001100111
///
/// This finds four unbroken edges:
///
///     00000111100100000
///         -   - -   ---
///     00001111001100111
fn count_horizontal_unbroken_edges_in_row(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    garden_index: usize,
    scan_y: i16,
    max_x: i16,
) -> usize {
    // On the left of the map, there is no garden.
    let mut last_top = false;
    let mut last_bottom = false;
    let mut unbroken_edges = 0;

    for x in 0..max_x {
        let new_top =
            try_find_merged_garden_index(original_garden_at_coordinate, merges, x, scan_y)
                == Some(garden_index);
        let new_bottom =
            try_find_merged_garden_index(original_garden_at_coordinate, merges, x, scan_y + 1)
                == Some(garden_index);

        let fully_ouside = !new_top && !new_bottom;
        let fully_insie = new_top && new_bottom;
        let same_as_before = last_top == new_top && last_bottom == new_bottom;

        if !same_as_before && !fully_ouside && !fully_insie {
            unbroken_edges += 1;
        }

        last_top = new_top;
        last_bottom = new_bottom;
    }

    unbroken_edges
}

fn count_vertical_unbroken_edges_in_row(
    original_garden_at_coordinate: &HashMap<(i16, i16), usize>,
    merges: &mut HashMap<usize, usize>,
    garden_index: usize,
    scan_x: i16,
    max_y: i16,
) -> usize {
    // On the left of the map, there is no garden.
    let mut last_top = false;
    let mut last_bottom = false;
    let mut unbroken_edges = 0;

    for y in 0..max_y {
        let new_top =
            try_find_merged_garden_index(original_garden_at_coordinate, merges, scan_x, y)
                == Some(garden_index);
        let new_bottom =
            try_find_merged_garden_index(original_garden_at_coordinate, merges, scan_x + 1, y)
                == Some(garden_index);

        let fully_ouside = !new_top && !new_bottom;
        let fully_insie = new_top && new_bottom;
        let same_as_before = last_top == new_top && last_bottom == new_bottom;

        if !same_as_before && !fully_ouside && !fully_insie {
            unbroken_edges += 1;
        }

        last_top = new_top;
        last_bottom = new_bottom;
    }

    unbroken_edges
}
