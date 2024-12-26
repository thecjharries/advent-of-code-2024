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

use memoize::memoize;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[memoize]
fn blink(number: usize, remaining: usize) -> usize {
    if 0 == remaining {
        return 1;
    }
    let stringified = number.to_string();
    if 0 == number {
        blink(1, remaining - 1)
    } else if 0 == stringified.len() % 2 {
        let half = stringified.len() / 2;
        let first_half = stringified[0..half].parse::<usize>().unwrap();
        let second_half = stringified[half..].parse::<usize>().unwrap();
        blink(first_half, remaining - 1) + blink(second_half, remaining - 1)
    } else {
        blink(2024 * number, remaining - 1)
    }
}

fn part1(input: String) -> usize {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| blink(x.parse::<usize>().unwrap(), 25))
        .sum()
}

fn part2(input: String) -> usize {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| blink(x.parse::<usize>().unwrap(), 75))
        .sum()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(55312, part1("125 17".to_string()))
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(65601038650482, part2("125 17".to_string()))
    }
}
