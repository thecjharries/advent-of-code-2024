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

fn part1(input: String) -> usize {
    let mut first = Vec::<usize>::new();
    let mut second = Vec::<usize>::new();
    for line in input.lines() {
        let numbers = line
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();
    return first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| first.abs_diff(*second))
        .sum();
}

fn part2(input: String) -> usize {
    let mut first = Vec::<usize>::new();
    let mut second = Vec::<usize>::new();
    for line in input.lines() {
        let numbers = line
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();
    return first
        .iter()
        .map(|first| second.iter().filter(|&second| second == first).count() * first)
        .sum();
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_handle_example() {
        assert_eq!(
            11,
            part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
                    .to_string(),
            )
        );
    }

    #[test]
    fn part2_should_handle_example() {
        assert_eq!(
            31,
            part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
                    .to_string(),
            )
        );
    }
}
