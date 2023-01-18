use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn beats(&self, other: &Action) -> bool {
        match self {
            Action::Rock => matches!(other, Action::Scissors),
            Action::Paper => matches!(other, Action::Rock),
            Action::Scissors => matches!(other, Action::Paper)
        }
    }

    fn loses(&self, other: &Action) -> bool {
        match self {
            Action::Rock => matches!(other, Action::Paper),
            Action::Paper => matches!(other, Action::Scissors),
            Action::Scissors => matches!(other, Action::Rock)
        }
    }

    fn get_defeat_action(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissors,
            Action::Scissors => Action::Rock,
        }
    }

    fn get_win_action(&self) -> Action {
        match self {
            Action::Rock => Action::Scissors,
            Action::Paper => Action::Rock,
            Action::Scissors => Action::Paper,
        }
    }

    fn get_match_score(&self, other: &Action) -> i32 {
        if self.beats(other) {
            6
        } else if self.loses(other) {
            0
        } else {
            3
        }
    }

    fn get_action_score(&self) -> i32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

fn get_match_history() -> HashMap<String, i32> {
    let input = fs::read_to_string("src/days/day2/assets/input.txt").expect("Error reading file");
    let mut matches : HashMap<String, i32> = HashMap::new();

    for line in input.lines() {
        let m: String = line.to_owned();
        if matches.contains_key(&m) {
            matches.insert(m.clone(), matches[&m] + 1);
        } else {
            matches.insert(m, 1);
        }
    }
    matches
}

pub fn part1() {
    let mut score: i32 = 0;
    let matches = get_match_history();
    for (key, value) in matches {
        let enemy_action: char = key.chars().nth(0).unwrap();
        let my_action: char = key.chars().last().unwrap();
        
        let enemy_action = match enemy_action {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissors,
            _ => panic!("Invalid enemy action"),
        };

        let my_action = match my_action {
            'X' => Action::Rock,
            'Y' => Action::Paper,
            'Z' => Action::Scissors,
            _ => panic!("Invalid my action"),
        };

        score += value * my_action.get_match_score(&enemy_action);
        score += value * my_action.get_action_score();
    }

    println!("Day 2 - Part 1: My score is {}", score);
}

pub fn part2() {
    let mut score: i32 = 0;
    let matches = get_match_history();
    for (key, value) in matches {
        let enemy_action: char = key.chars().nth(0).unwrap();
        let match_result: char = key.chars().last().unwrap();
        
        let enemy_action = match enemy_action {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissors,
            _ => panic!("Invalid enemy action"),
        };

        let my_action = match match_result {
            'X' => enemy_action.get_win_action(),
            'Z' => enemy_action.get_defeat_action(),
            'Y' => enemy_action.clone(),
            _ => panic!("Invalid match result"),
        };

        score += value * my_action.get_match_score(&enemy_action);
        score += value * my_action.get_action_score();
    }

    println!("Day 2 - Part 2: My score is {}", score);
}