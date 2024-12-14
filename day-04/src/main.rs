// Copyright 2024 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn convert_input_to_2d_array(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: String) -> usize {
    let grid = convert_input_to_2d_array(input);
    let mut count = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if 'X' == grid[row][column] {
                for row_offset in -1..=1 {
                    for column_offset in -1..=1 {
                        if row_offset == 0 && column_offset == 0 {
                            continue;
                        }
                        let new_row = row as i32 + row_offset;
                        let new_column = column as i32 + column_offset;
                        if new_row < 0
                            || new_row >= grid.len() as i32
                            || new_column < 0
                            || new_column >= grid[row].len() as i32
                        {
                            continue;
                        }
                        if 'M' != grid[new_row as usize][new_column as usize] {
                            continue;
                        }
                        let new_row = row as i32 + 2 * row_offset;
                        let new_column = column as i32 + 2 * column_offset;
                        if new_row < 0
                            || new_row >= grid.len() as i32
                            || new_column < 0
                            || new_column >= grid[row].len() as i32
                        {
                            continue;
                        }
                        if 'A' != grid[new_row as usize][new_column as usize] {
                            continue;
                        }
                        let new_row = row as i32 + 3 * row_offset;
                        let new_column = column as i32 + 3 * column_offset;
                        if new_row < 0
                            || new_row >= grid.len() as i32
                            || new_column < 0
                            || new_column >= grid[row].len() as i32
                        {
                            continue;
                        }
                        if 'S' != grid[new_row as usize][new_column as usize] {
                            continue;
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_array_conversion_works() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA"
            .to_string();
        let expected = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        ];
        assert_eq!(expected, convert_input_to_2d_array(input));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            18,
            part1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                    .to_string()
            )
        )
    }
}
