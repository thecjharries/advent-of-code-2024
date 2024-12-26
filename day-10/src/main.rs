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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn get_neighbors(coordinate: Coordinate, grid: &Vec<Vec<usize>>) -> HashSet<Coordinate> {
    if 9 == grid[coordinate.y][coordinate.x] {
        return HashSet::new();
    }
    let mut neighbors = HashSet::new();
    if 0 < coordinate.x
        && grid[coordinate.y][coordinate.x - 1] == grid[coordinate.y][coordinate.x] + 1
    {
        neighbors.insert(Coordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        });
    }
    if 0 < coordinate.y
        && grid[coordinate.y - 1][coordinate.x] == grid[coordinate.y][coordinate.x] + 1
    {
        neighbors.insert(Coordinate {
            x: coordinate.x,
            y: coordinate.y - 1,
        });
    }
    if coordinate.x < grid[0].len() - 1
        && grid[coordinate.y][coordinate.x + 1] == grid[coordinate.y][coordinate.x] + 1
    {
        neighbors.insert(Coordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        });
    }
    if coordinate.y < grid.len() - 1
        && grid[coordinate.y + 1][coordinate.x] == grid[coordinate.y][coordinate.x] + 1
    {
        neighbors.insert(Coordinate {
            x: coordinate.x,
            y: coordinate.y + 1,
        });
    }
    neighbors
}

fn find_trail_ends(start: Coordinate, grid: &Vec<Vec<usize>>) -> HashSet<Coordinate> {
    let mut trail_ends = HashSet::new();
    if 9 == grid[start.y][start.x] {
        trail_ends.insert(start);
    } else {
        for neighbor in get_neighbors(start, grid) {
            trail_ends.extend(find_trail_ends(neighbor, grid));
        }
    }
    return trail_ends;
}

fn part1(input: String) -> usize {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut trailheads: Vec<Coordinate> = Vec::new();
    for (y, line) in input.trim().lines().enumerate() {
        grid.push(Vec::new());
        for (x, character) in line.chars().enumerate() {
            grid[y].push(character.to_digit(10).unwrap() as usize);
            if 0 == grid[y][x] {
                trailheads.push(Coordinate { x, y });
            }
        }
    }
    let mut trails = 0;
    for trailhead in trailheads {
        trails += find_trail_ends(trailhead, &grid).len();
    }
    trails
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_neighbors() {
        let grid = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 0, 1],
            vec![2, 3, 4, 5],
        ];
        let coordinate = Coordinate { x: 1, y: 1 };
        let expected: HashSet<Coordinate> = vec![Coordinate { x: 2, y: 1 }].into_iter().collect();
        assert_eq!(expected, get_neighbors(coordinate, &grid));
    }

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            1,
            part1(
                "0123
1234
8765
9876"
                    .to_string()
            )
        );
        assert_eq!(
            36,
            part1(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
                    .to_string()
            )
        )
    }
}
