#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::iter::repeat;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<_> = (0..size).map(|_| vec![0; size as usize]).collect();

    if size == 0 {
        return matrix;
    }

    // Fill the first row from (0,0) to (n,0) with [1, n], where n == size.
    for i in 0..size {
        matrix[0][i as usize] = i + 1;
    }

    let mut position = [0, (size - 1) as i32];

    let mut directions = [[1, 0], [0, -1], [-1, 0], [0, 1]].iter().cycle();

    let mut direction = directions.next().unwrap();

    let segment_lengths: Vec<_> = (1..size).rev().flat_map(|i| repeat(i).take(2)).collect();
    let mut last_recorded_number = size;

    for length in segment_lengths { 
        for number in last_recorded_number+1 ..= last_recorded_number+length {
            position = [position[0] + direction[0], position[1] + direction[1]];
            let [row, column] = position;
            matrix[row as usize][column as usize] = number;
        }

        last_recorded_number += length;
        direction = directions.next().unwrap();
    }

    matrix
}
