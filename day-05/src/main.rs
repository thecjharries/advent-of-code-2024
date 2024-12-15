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

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    number: usize,
    before: Vec<usize>,
}

impl Rule {
    pub fn new_from_line(line: &str) -> Self {
        let mut parts = line.split("|");
        let number: usize = parts.next().unwrap().parse().unwrap();
        let before_entry: usize = parts.next().unwrap().parse().unwrap();
        return Rule {
            number,
            before: vec![before_entry],
        };
    }

    pub fn add_before_from_entry(&mut self, entry: &str) {
        let mut parts = entry.split("|");
        let number: usize = parts.next().unwrap().parse().unwrap();
        if number != self.number {
            return;
        }
        let before_entry: usize = parts.next().unwrap().parse().unwrap();
        self.before.push(before_entry);
    }

    pub fn add_before_from_rule(&mut self, rule: Rule) {
        if rule.number != self.number {
            return;
        }
        self.before.extend(rule.before.iter());
    }
}

fn part1(input: String) -> usize {
    let input = input.trim();
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap().split("\n");
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for rule_line in rules_section {
        let mut rule = Rule::new_from_line(rule_line);
        if rules.contains_key(&rule.number) {
            let existing_rule = rules.get(&rule.number).unwrap();
            rule.add_before_from_rule(existing_rule.clone());
            rules.insert(rule.number, rule);
        } else {
            rules.insert(rule.number, rule);
        }
    }
    let messages = sections.next().unwrap().split("\n");
    let mut count = 0;
    for message in messages {
        let sequence: Vec<usize> = message.split(",").map(|x| x.parse().unwrap()).collect();
        let mut valid = true;
        for i in 0..sequence.len() - 1 {
            let current = sequence[i];
            if !rules.contains_key(&current) {
                valid = false;
                break;
            }
            let rule = rules.get(&current).unwrap();
            let remaining = &sequence[i + 1..];
            for next in remaining {
                if !rule.before.contains(next) {
                    valid = false;
                    break;
                    print!("{} -> {} is invalid\n", current, next);
                }
            }
            if !valid {
                break;
            }
        }
        if valid {
            count += sequence[sequence.len() / 2];
        }
    }
    count
}

fn part2(input: String) -> usize {
    let input = input.trim();
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap().split("\n");
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for rule_line in rules_section {
        let mut rule = Rule::new_from_line(rule_line);
        if rules.contains_key(&rule.number) {
            let existing_rule = rules.get(&rule.number).unwrap();
            rule.add_before_from_rule(existing_rule.clone());
            rules.insert(rule.number, rule);
        } else {
            rules.insert(rule.number, rule);
        }
    }
    let messages = sections.next().unwrap().split("\n");
    let mut available_messages = Vec::new();
    for message in messages {
        let sequence: Vec<usize> = message.split(",").map(|x| x.parse().unwrap()).collect();
        let mut valid = true;
        for i in 0..sequence.len() - 1 {
            let current = sequence[i];
            if !rules.contains_key(&current) {
                valid = false;
                break;
            }
            let rule = rules.get(&current).unwrap();
            let remaining = &sequence[i + 1..];
            for next in remaining {
                if !rule.before.contains(next) {
                    valid = false;
                    break;
                    print!("{} -> {} is invalid\n", current, next);
                }
            }
            if !valid {
                break;
            }
        }
        if !valid {
            available_messages.push(sequence);
        }
    }
    let mut count = 0;
    for message in available_messages {
        let mut message_rules = message
            .iter()
            .map(|x| {
                if rules.contains_key(x) {
                    return rules.get(x).unwrap().clone();
                }
                return Rule {
                    number: *x,
                    before: Vec::new(),
                };
            })
            .collect::<Vec<Rule>>();
        message_rules.sort_by(|a, b| {
            if a.number == b.number {
                return Ordering::Equal;
            }
            if a.before.contains(&b.number) {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        count += message_rules[message_rules.len() / 2].number;
    }
    count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_new_from_line() {
        let rule = Rule::new_from_line("47|53");
        assert_eq!(47, rule.number);
        assert_eq!(vec![53], rule.before);
    }

    #[test]
    fn test_rule_add_before() {
        let mut rule = Rule::new_from_line("47|53");
        rule.add_before_from_entry("97|13");
        assert_eq!(vec![53], rule.before);
        rule.add_before_from_entry("47|13");
        assert_eq!(vec![53, 13], rule.before);
    }

    #[test]
    fn test_rule_add_rule() {
        let mut rule = Rule::new_from_line("47|53");
        let other_rule = Rule::new_from_line("47|13");
        rule.add_before_from_rule(other_rule);
        assert_eq!(vec![53, 13], rule.before);
        let other_rule = Rule::new_from_line("53|29");
        rule.add_before_from_rule(other_rule);
        assert_eq!(vec![53, 13], rule.before);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            143,
            part1(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            123,
            part2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
                    .to_string()
            )
        );
    }
}
