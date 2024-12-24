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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Partition {
    Block(usize, usize),
    Empty(usize),
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
    let mut filesystem: Vec<Partition> = Vec::new();
    for number in numbers.iter() {
        if is_block {
            filesystem.push(Partition::Block(*number, index));
            index += 1;
        } else {
            filesystem.push(Partition::Empty(*number));
        }
        is_block = !is_block;
    }
    let mut current_move_index = index - 1;
    while current_move_index > 0 {
        let mut right_index = filesystem.len() - 1;
        loop {
            match filesystem[right_index] {
                Partition::Empty(_) => {
                    right_index -= 1;
                }
                Partition::Block(_, block_index) => {
                    if block_index == current_move_index {
                        break;
                    } else {
                        right_index -= 1;
                    }
                }
            }
        }
        let mut left_index = 0;
        while left_index < right_index {
            match filesystem[left_index] {
                Partition::Empty(empty_size) => {
                    if let Partition::Block(block_size, block_index) = filesystem[right_index] {
                        if empty_size >= block_size {
                            let mut new_filesystem = filesystem[..left_index].to_vec();
                            new_filesystem.push(Partition::Block(block_size, block_index));
                            if empty_size > block_size {
                                new_filesystem.push(Partition::Empty(empty_size - block_size));
                            }
                            new_filesystem.extend(filesystem[left_index + 1..right_index].to_vec());
                            new_filesystem.push(Partition::Empty(block_size));
                            new_filesystem.extend(filesystem[right_index + 1..].to_vec());
                            filesystem = new_filesystem;
                        }
                    }
                    left_index += 1;
                }
                Partition::Block(_, _) => {
                    left_index += 1;
                }
            }
        }
        current_move_index -= 1;
    }
    let mut index = 0;
    filesystem.into_iter().fold(0, |acc, x| match x {
        Partition::Empty(size) => {
            index += size;
            acc
        }
        Partition::Block(size, contents) => {
            let mut sum = acc;
            for _ in 0..size {
                sum += index * contents;
                index += 1;
            }
            sum
        }
    })
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1() {
        assert_eq!(1928, part1("2333133121414131402".to_string()))
    }

    #[test]
    fn it_solves_part2() {
        assert_eq!(2858, part2("2333133121414131402".to_string()))
    }
}
