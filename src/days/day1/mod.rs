use std::fs;

fn get_calories(input_path: &str) -> Vec<i32> {
    let contents: String = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    let mut calories: Vec<i32> = contents
        .split("\n\n")
        .map(|lines| lines
            .split("\n")
            .map(|line| line.parse::<i32>().unwrap())
            .sum())
        .collect();
    
    calories.sort();
    calories.reverse();
    calories
}

pub fn part1() -> i32 {
    let input_path: &str = "src/days/day1/assets/input.txt";
    let calories: Vec<i32> = get_calories(input_path);
    println!("Part 1: The highest colory");
    calories[0]
}

pub fn part2(top: usize) -> i32 {
    let input_path: &str = "src/days/day1/assets/input.txt";
    let calories: Vec<i32> = get_calories(input_path);
    println!("Part 2: Sum of top {} calories", top);
    calories[0..top].iter().sum::<i32>()
}