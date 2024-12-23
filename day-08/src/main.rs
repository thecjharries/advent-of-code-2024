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

use std::collections::HashMap;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn parse_part1_map(input: String) -> HashMap<char, Vec<Coordinate>> {
    let mut map = HashMap::new();
    let lines = input.trim().split("\n");
    for (y, line) in lines.enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character.is_alphanumeric() {
                let coordinates = map.entry(character).or_insert_with(Vec::new);
                coordinates.push(Coordinate::new(x as i32, y as i32));
            }
        }
    }
    map
}

fn part1(input: String) -> usize {
    todo!()
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_coordinates() {
        assert_eq!(Coordinate { x: 1, y: 2 }, Coordinate::new(1, 2));
    }

    #[test]
    fn it_creates_coordinate_hashmaps() {
        let mut expected = HashMap::new();
        expected.insert(
            '0',
            vec![
                Coordinate::new(8, 1),
                Coordinate::new(5, 2),
                Coordinate::new(7, 3),
                Coordinate::new(4, 4),
            ],
        );
        expected.insert(
            'A',
            vec![
                Coordinate::new(6, 5),
                Coordinate::new(8, 8),
                Coordinate::new(9, 9),
            ],
        );
        assert_eq!(
            expected,
            parse_part1_map(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
                    .to_string()
            )
        );
    }

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            14,
            part1(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
                    .to_string()
            )
        );
    }
}
