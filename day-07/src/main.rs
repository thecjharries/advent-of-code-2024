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

fn solve_equation(solution: usize, numbers: Vec<usize>) -> bool {
    if 2 == numbers.len() {
        if solution == numbers[0] + numbers[1] {
            return true;
        }
        if solution == numbers[0] * numbers[1] {
            return true;
        }
        return false;
    }
    let mut plus_numbers = vec![numbers[0] + numbers[1]];
    plus_numbers.extend_from_slice(&numbers[2..]);
    if solve_equation(solution, plus_numbers) {
        return true;
    }
    let mut times_numbers = vec![numbers[0] * numbers[1]];
    times_numbers.extend_from_slice(&numbers[2..]);
    if solve_equation(solution, times_numbers) {
        return true;
    }
    false
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
    fn it_recursively_solves_equations() {
        assert!(solve_equation(190, vec![10, 19]));
        assert!(!solve_equation(83, vec![17, 5]));
    }

    #[test]
    fn it_solves_part1() {
        assert_eq!(
            3749,
            part1(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
                    .to_string()
            )
        );
    }
}
