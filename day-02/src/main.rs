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

fn parse_level_str(level: &str) -> Vec<usize> {
    level
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|part| part.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn is_level_safe(levels: Vec<usize>) -> bool {
    for index in 0..levels.len() - 1 {
        if 3 < levels[index].abs_diff(levels[index + 1]) || levels[index] == levels[index + 1] {
            return false;
        }
    }
    let mut sorted = levels.clone();
    sorted.sort();
    if levels[0] > levels[1] && levels[1] > levels[2] {
        sorted.reverse();
    }
    return levels == sorted;
}

fn part1(input: String) -> usize {
    input
        .lines()
        .filter(|line| is_level_safe(parse_level_str(line)))
        .count()
}

fn part2(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels = parse_level_str(line);
            if is_level_safe(levels.clone()) {
                return true;
            } else {
                let mut safe = false;
                for index in 0..levels.len() {
                    let mut removed = levels[0..index].to_vec();
                    removed.extend(levels[index + 1..].to_vec());
                    if is_level_safe(removed) {
                        safe = true;
                        break;
                    }
                }
                return safe;
            }
        })
        .count()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_level_safe() {
        assert!(is_level_safe(parse_level_str("7 6 4 2 1")));
        assert!(!is_level_safe(parse_level_str("1 2 7 8 9")));
        assert!(!is_level_safe(parse_level_str("9 7 6 2 1")));
        assert!(!is_level_safe(parse_level_str("1 3 2 4 5")));
        assert!(!is_level_safe(parse_level_str("8 6 4 4 1")));
        assert!(is_level_safe(parse_level_str("1 3 6 7 9")));
    }

    #[test]
    fn test_part1() {
        assert!(part1("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9".to_string()) == 2);
    }

    #[test]
    fn test_part2() {
        assert!(part2("7 6 4 2 1".to_string()) > 0);
        assert!(part2("1 2 7 8 9".to_string()) == 0);
        assert!(part2("9 7 6 2 1".to_string()) == 0);
        assert!(part2("1 3 2 4 5".to_string()) > 0);
        assert!(part2("8 6 4 4 1".to_string()) > 0);
        assert!(part2("1 3 6 7 9".to_string()) > 0);
    }
}
