use std::fs;
use std::collections::HashMap;

struct R {
    rucksacks: Vec<String>,
}

impl R {
    fn new(rucksacks: Vec<String>) -> R {
        R {
            rucksacks: rucksacks
        }
    }

    fn get_common_item(&self) -> char {
        let mut common_items: HashMap<char, usize> = HashMap::new();
        for r in &self.rucksacks {
            let mut set = r.chars().collect::<Vec<char>>();
            set.sort();
            set.dedup();
            for c in set {
                let count = common_items.get(&c).unwrap_or(&0);
                common_items.insert(c, (1 + *count) as usize);
            }
        }
        for (item, count) in common_items {
            if count == self.rucksacks.len() {
                return item
            }
        }
        unreachable!();
    }
}

fn get_input(input_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

fn get_priorities() -> HashMap<char, usize> {
    ('a'..='z').chain('A'..='Z').enumerate().map(|(i, c)| (c, i + 1)).collect::<HashMap<char, usize>>()
}

pub fn part1() {
    fn split_string(s: String) -> Vec<String> {
        s.chars()
        .collect::<Vec<char>>()
        .chunks(s.len() / 2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
    }

    let input = get_input("src/days/day3/assets/input.txt");
    let priorities = get_priorities();
    let total_priority : usize = input
        .iter()
        .map(|r| R::new(split_string(r.to_string())).get_common_item())
        .map(|c| priorities.get(&c).unwrap())
        .sum();
    
    println!("Day 3 - Part 1: Total priority is {}", total_priority);
}

pub fn part2() {
    let input = get_input("src/days/day3/assets/input.txt");
    let priorities = get_priorities();
    let total_priority : usize = input
        .chunks(3)
        .map(|r| R::new((*r).to_vec()).get_common_item())
        .map(|c| priorities.get(&c).unwrap())
        .sum();
    
    println!("Day 3 - Part 2: Total priority is {}", total_priority);
}