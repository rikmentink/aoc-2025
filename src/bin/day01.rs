use aoc_2025::read_input;

const SAFE_SIZE: i32 = 100;
const START_POSITION: i32 = 50;

enum Direction {
    L,
    R,
}

fn main() {
    println!("Day 1 - Safe code");
    let input = read_input(1).unwrap();

    // Use a functional pipeline to:
    // 1. Parse lines into (Direction, Distance)
    // 2. Scan through rotations, maintaining current position state
    // 3. Filter for positions that equal 0
    // 4. Count them
    let zero_positions = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction = match direction {
                "L" => Direction::L,
                "R" => Direction::R,
                _ => unreachable!("Invalid direction found in input"),
            };
            let distance = distance.parse::<i32>().expect("Invalid distance number");
            (direction, distance)
        })
        .scan(START_POSITION, |current_pos, (direction, distance)| {
            *current_pos = shift_position(*current_pos, direction, distance);
            Some(*current_pos)
        })
        .filter(|&pos| pos == 0)
        .count();

    println!("The code is {}", zero_positions);
}

/**
 * Helper function to shift the position correctly, including overflow.
 */
fn shift_position(current_position: i32, direction: Direction, distance: i32) -> i32 {
    let delta = match direction {
        Direction::L => -distance,
        Direction::R => distance,
    };

    // Use Euclid's division to wrap the position around the safe size.
    (current_position + delta).rem_euclid(SAFE_SIZE)
}
