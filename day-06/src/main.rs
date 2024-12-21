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

use std::collections::HashSet;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GridContent {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Grid {
    contents: Vec<Vec<GridContent>>,
    starting_point: Coordinate,
    visited: HashSet<Coordinate>,
}

impl Grid {
    fn new(input: String) -> Self {
        let input = input.trim();
        let mut contents = Vec::new();
        let mut starting_point = Coordinate::new(0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                match character {
                    '.' => row.push(GridContent::Empty),
                    '#' => row.push(GridContent::Wall),
                    '^' => {
                        row.push(GridContent::Empty);
                        starting_point = Coordinate::new(x, y);
                    }
                    _ => {}
                }
            }
            contents.push(row);
        }
        Self {
            contents,
            starting_point,
            visited: HashSet::new(),
        }
    }
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
    fn it_creates_coordinates() {
        assert_eq!(Coordinate::new(0, 0), Coordinate { x: 0, y: 0 });
    }

    #[test]
    fn it_creates_grids() {
        let input = "########".to_string();
        let grid = Grid::new(input);
        assert_eq!(Coordinate::new(0, 0), grid.starting_point);
        assert_eq!(GridContent::Wall, grid.contents[0][0]);
        assert_eq!(0, grid.visited.len());
    }

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            41,
            part1(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                    .to_string()
            )
        )
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(
            41,
            part2(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                    .to_string()
            )
        )
    }
}
