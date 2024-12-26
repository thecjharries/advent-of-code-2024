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
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for _ in 0..25 {
        let mut new_numbers = Vec::new();
        for element in numbers.iter() {
            if 0 == *element {
                new_numbers.push(1);
            } else if 0 == element.to_string().len() % 2 {
                let characters = element.to_string().chars().collect::<Vec<char>>();
                let half = characters.len() / 2;
                let first_half = characters[0..half]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let second_half = characters[half..]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                new_numbers.push(first_half);
                new_numbers.push(second_half);
            } else {
                new_numbers.push(2024 * *element);
            }
        }
        numbers = new_numbers;
    }
    numbers.len()
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(55312, part1("125 17".to_string()))
    }
}
