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

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn walk(&self, coordinate: &Coordinate) -> Coordinate {
        let (dx, dy) = match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        };
        Coordinate::new(coordinate.x + dx, coordinate.y + dy)
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum GridContent {
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
struct Grid {
    contents: Vec<Vec<GridContent>>,
    current_position: Coordinate,
    visited: HashSet<Coordinate>,
    path: Vec<Coordinate>,
}

impl Grid {
    fn new(input: String) -> Self {
        let input = input.trim();
        let mut contents = Vec::new();
        let mut current_position = Coordinate::new(0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                match character {
                    '.' => row.push(GridContent::Empty),
                    '#' => row.push(GridContent::Wall),
                    '^' => {
                        row.push(GridContent::Empty);
                        current_position =
                            Coordinate::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                    }
                    _ => {}
                }
            }
            contents.push(row);
        }
        Self {
            contents,
            current_position: current_position.clone(),
            visited: HashSet::new(),
            path: vec![current_position.clone()],
        }
    }

    fn run(&mut self) -> usize {
        let mut direction = Direction::North;
        loop {
            let next_position = direction.walk(&self.current_position);
            self.visited.insert(self.current_position.clone());
            if next_position.x < 0
                || next_position.y < 0
                || next_position.y as usize >= self.contents.len()
                || next_position.x as usize >= self.contents[next_position.y as usize].len()
            {
                break;
            }
            match self.contents[next_position.y as usize][next_position.x as usize] {
                GridContent::Empty => {
                    self.current_position = next_position;
                }
                GridContent::Wall => {
                    direction = direction.turn_right();
                }
            }
        }
        return self.visited.len();
    }

    fn run_part2(&mut self) -> usize {
        let mut direction = Direction::North;
        let mut new_obstacles: HashSet<Coordinate> = HashSet::new();
        let possible_obstacle_grid = self.clone();
        loop {
            self.visited.insert(self.current_position.clone());
            if self.current_position != self.path[self.path.len() - 1] {
                self.path.push(self.current_position.clone());
            }
            let next_position = direction.walk(&self.current_position);
            if next_position.x < 0
                || next_position.y < 0
                || next_position.y as usize >= self.contents.len()
                || next_position.x as usize >= self.contents[next_position.y as usize].len()
            {
                break;
            }
            match self.contents[next_position.y as usize][next_position.x as usize] {
                GridContent::Empty => {
                    self.current_position = next_position;
                }
                GridContent::Wall => {
                    direction = direction.turn_right();
                }
            }
        }
        for possible_obstacle_index in 3..self.path.len() {
            let mut new_grid = possible_obstacle_grid.clone();
            if new_grid.current_position == self.path[possible_obstacle_index] {
                continue;
            }
            new_grid.contents[self.path[possible_obstacle_index].y as usize]
                [self.path[possible_obstacle_index].x as usize] = GridContent::Wall;
            let mut direction = Direction::North;
            let mut cycle = true;
            let mut current_repetitions = 0;
            loop {
                if new_grid.visited.contains(&new_grid.current_position) {
                    current_repetitions += 1;
                    if current_repetitions > new_grid.visited.len() {
                        break;
                    }
                }
                new_grid.visited.insert(new_grid.current_position.clone());
                let next_position = direction.walk(&new_grid.current_position);
                if next_position.x < 0
                    || next_position.y < 0
                    || next_position.y as usize >= new_grid.contents.len()
                    || next_position.x as usize >= new_grid.contents[next_position.y as usize].len()
                {
                    cycle = false;
                    break;
                }
                match new_grid.contents[next_position.y as usize][next_position.x as usize] {
                    GridContent::Empty => {
                        new_grid.current_position = next_position;
                    }
                    GridContent::Wall => {
                        direction = direction.turn_right();
                    }
                }
            }
            if cycle {
                new_obstacles.insert(self.path[possible_obstacle_index].clone());
            }
        }
        new_obstacles.len() - 1
    }
}

fn part1(input: String) -> usize {
    let mut grid = Grid::new(input);
    grid.run()
}

fn part2(input: String) -> usize {
    let mut grid = Grid::new(input);
    grid.run_part2()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_turns_right() {
        assert_eq!(Direction::East, Direction::North.turn_right());
        assert_eq!(Direction::South, Direction::East.turn_right());
        assert_eq!(Direction::West, Direction::South.turn_right());
        assert_eq!(Direction::North, Direction::West.turn_right());
    }

    #[test]
    fn it_creates_coordinates() {
        assert_eq!(Coordinate::new(0, 0), Coordinate { x: 0, y: 0 });
    }

    #[test]
    fn it_creates_grids() {
        let input = "########".to_string();
        let grid = Grid::new(input);
        assert_eq!(Coordinate::new(0, 0), grid.current_position);
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
            6,
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
