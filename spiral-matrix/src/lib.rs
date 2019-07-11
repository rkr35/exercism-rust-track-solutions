#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::iter::repeat;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<_> = (0..size).map(|_| vec![0; size as usize]).collect();

    // Cycle of right, down, left, and up directions.
    let mut directions = [[0, 1], [1, 0], [0, -1], [-1, 0]].iter().cycle();
    let mut direction = directions.next().unwrap();

    // The position in the matrix of the next number to record.
    // We offset by the initial direction in order to place the first number
    // correctly.
    let mut position = [-direction[0], -direction[1]];

    // Keeps track of the last number placed in the matrix.
    let mut last_recorded_number = 0;

    // Holds the length of each segment before changing directions.
    // After the first row, segments come in pairs with the same length starting from n-1.
    // For example, for n=5, the first two segments after the first segment of length 5 will have length 4. 
    // Then the next two segments have length 3, and so on. The last two segments have length 1.
    let segment_lengths: Vec<_> = (1..=size).rev().flat_map(|i| repeat(i).take(2)).skip(1).collect();

    for length in segment_lengths {
        // For each number belonging to this segment...
        for number in last_recorded_number + 1..=last_recorded_number + length {
            // Update the position for this number.
            position = [position[0] + direction[0],
                        position[1] + direction[1]];

            // Place the number in the matrix at that position.
            let [row, column] = position;
            matrix[row as usize][column as usize] = number;
        }

        last_recorded_number += length;

        // Now that we filled this segment, we'll move onto the next segment, which,
        // as mentioned above, will have a different direction.
        direction = directions.next().unwrap();
    }

    matrix
}
