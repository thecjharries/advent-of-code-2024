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

#[derive(Debug, PartialEq, Eq, Clone)]
enum File {
    Empty,
    Block(usize),
}

fn part1(input: String) -> usize {
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|x| {
            x.to_string()
                .parse::<usize>()
                .expect("Unable to parse number")
        })
        .collect();
    let mut index = 0;
    let mut is_block = true;
    let mut filesystem: Vec<File> = Vec::new();
    for number in numbers.iter() {
        if is_block {
            let new_file = vec![File::Block(index); *number];
            filesystem.extend(new_file);
            index += 1;
        } else {
            let empty_space = vec![File::Empty; *number];
            filesystem.extend(empty_space);
        }
        is_block = !is_block;
    }
    let mut left_index = 0;
    let mut right_index = filesystem.len() - 1;
    while left_index < right_index {
        match filesystem[left_index] {
            File::Empty => match filesystem[right_index] {
                File::Empty => {
                    right_index -= 1;
                }
                File::Block(_) => {
                    filesystem.swap(left_index, right_index);
                    left_index += 1;
                    right_index -= 1;
                }
            },
            File::Block(_) => {
                left_index += 1;
            }
        }
    }
    let mut index = 0;
    filesystem.into_iter().fold(0, |acc, x| {
        acc + match x {
            File::Empty => 0,
            File::Block(contents) => {
                index += 1;
                (index - 1) * contents
            }
        }
    })
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
        assert_eq!(1928, part1("2333133121414131402".to_string()))
    }
}
