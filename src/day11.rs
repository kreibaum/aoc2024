
use std::collections::HashMap;

pub fn main() {
    let mut memory : HashMap<(u64, u64), u64> = HashMap::new();

    println!("Running example tasks for 6 blinks:");
    println!("125 after 6 blinks is {} stones.", count_stones(125, 6, &mut memory)); // == 7
    println!("17 after 6 blinks is {} stones.", count_stones(17, 6, &mut memory)); // == 15

    // Actual task: 28591 78 0 3159881 4254 524155 598 1
    // Actual blinks: 25

    println!("Running actual task for 25 blinks:");
    println!("28591 after 25 blinks is {} stones.", count_stones(28591, 25, &mut memory));
    println!("78 after 25 blinks is {} stones.", count_stones(78, 25, &mut memory));
    println!("0 after 25 blinks is {} stones.", count_stones(0, 25, &mut memory));
    println!("3159881 after 25 blinks is {} stones.", count_stones(3159881, 25, &mut memory));
    println!("4254 after 25 blinks is {} stones.", count_stones(4254, 25, &mut memory));
    println!("524155 after 25 blinks is {} stones.", count_stones(524155, 25, &mut memory));
    println!("598 after 25 blinks is {} stones.", count_stones(598, 25, &mut memory));
    println!("1 after 25 blinks is {} stones.", count_stones(1, 25, &mut memory));

    // Sum everything up
    let mut total = 0;
    total += count_stones(28591, 25, &mut memory);
    total += count_stones(78, 25, &mut memory);
    total += count_stones(0, 25, &mut memory);
    total += count_stones(3159881, 25, &mut memory);
    total += count_stones(4254, 25, &mut memory);
    total += count_stones(524155, 25, &mut memory);
    total += count_stones(598, 25, &mut memory);
    total += count_stones(1, 25, &mut memory);
    println!("Total stones: {}", total);
    println!("Memory used: {}", memory.len());

    // Part two, blink 75 times.

    let mut total = 0;
    total += count_stones(28591, 75, &mut memory);
    total += count_stones(78, 75, &mut memory);
    total += count_stones(0, 75, &mut memory);
    total += count_stones(3159881, 75, &mut memory);
    total += count_stones(4254, 75, &mut memory);
    total += count_stones(524155, 75, &mut memory);
    total += count_stones(598, 75, &mut memory);
    total += count_stones(1, 75, &mut memory);
    println!("Total stones: {}", total);
    println!("Memory used: {}", memory.len());

}


fn count_stones( label : u64, blinks: u64, memory: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(&result) = memory.get(&(label, blinks)) {
        return result;
    }
    let result = if label == 0 {
        count_stones(1, blinks - 1, memory)
    } else if let Some((first_half, second_half)) = split_label(label) {
        count_stones(first_half, blinks - 1, memory) + count_stones(second_half, blinks - 1, memory)
    } else {
        count_stones(label * 2024, blinks - 1, memory)
    };

    memory.insert((label, blinks), result);
    result
}

fn split_label( label: u64 ) -> Option<(u64, u64)> {
    // Find out if the label has an even number of decimal digits
    let label_str = label.to_string();
    if label_str.len() % 2 != 0 {
        return None;
    }
    let half_len = label_str.len() / 2;
    let first_half = label_str[..half_len].parse().unwrap();
    let second_half = label_str[half_len..].parse().unwrap();
    return Some((first_half, second_half));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_label() {
        assert_eq!(split_label(125), None);
        assert_eq!(split_label(17), Some((1, 7)));
        assert_eq!(split_label(1), None);
        assert_eq!(split_label(12), Some((1, 2)));
        assert_eq!(split_label(1234), Some((12, 34)));
        assert_eq!(split_label(1000), Some((10, 00)));
    }
}