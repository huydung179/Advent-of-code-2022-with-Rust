use std::fs;

fn get_calories(input_path: &str) -> Vec<i32> {
    let contents: String = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    let mut calories: Vec<i32> = contents
        .split("\r\n")
        .map(|lines| lines
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum())
        .collect();
    
    calories.sort();
    calories.reverse();
    calories
}

pub fn part1() {
    let input_path: &str = "src/days/day1/assets/input.txt";
    let calories: Vec<i32> = get_calories(input_path);
    println!("Day 1 - Part 1: The highest colory is {}", calories[0]);
}

pub fn part2(top: usize) {
    let input_path: &str = "src/days/day1/assets/input.txt";
    let calories: Vec<i32> = get_calories(input_path);
    println!("Day 1 - Part 2: Sum of top {} calories is {}", top, calories[0..top].iter().sum::<i32>());
}