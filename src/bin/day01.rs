use aoc_2025::read_input;

enum Direction {
    L,
    R,
}

fn main() {
    println!("Day 1 - Safe code");
    let input = read_input(1).unwrap();

    // Parse the input into directions and distances
    let rotations: Vec<(Direction, i32)> = input
        .lines()
        .map(|line| {
            // Match the direction string to the Direction enum
            let (direction_str, distance_str) = line.split_at(1);
            let direction = match direction_str {
                "L" => Direction::L,
                "R" => Direction::R,
                _ => panic!("Invalid direction: {}", direction_str),
            };

            // Parse the distance into an integer
            let distance: i32 = distance_str
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Invalid distance: {}", distance_str));
            (direction, distance)
        })
        .collect();

    // Follow the rotations and keep track of the position.
    let start_position = 50;
    let mut current_position = start_position;
    let mut zero_positions = 0;

    for (direction, distance) in rotations {
        current_position = shift_position(current_position, direction, distance);
        if current_position == 0 {
            zero_positions += 1;
        }
    }

    // The answer is the code of the safe: the amount of zero positions.
    println!("The code is {}", zero_positions);
}

/**
 * Helper function to shift the position based on direction and distance,
 * wrapping around the 0..99 range.
 */
fn shift_position(current_position: i32, direction: Direction, distance: i32) -> i32 {
    // Calculate the shift in position.
    let delta = match direction {
        Direction::L => -distance,
        Direction::R => distance,
    };

    // Calculate the new position by adding the difference to the current position.
    let mut new_position = current_position + delta;
    new_position = new_position % 100;
    if new_position < 0 {
        new_position += 100;
    }
    new_position
}
