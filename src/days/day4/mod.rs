use std::fs;

#[derive(Clone, Copy)]
struct Assignement {
    from: u32,
    to: u32,
}

struct ElvesAssignment {
    left: Assignement,
    right: Assignement,
}

impl ElvesAssignment {
    fn is_fully_contain(&self) -> bool {
        (self.left.from <= self.right.from && self.left.to >= self.right.to) ||
        (self.right.from <= self.left.from && self.right.to >= self.left.to) 
    }

    fn is_overlap(&self) -> bool {
        (self.left.from <= self.right.from && self.left.to >= self.right.from) ||
        (self.right.from <= self.left.from && self.right.to >= self.left.from) 
    }
}

fn get_input() -> Vec<ElvesAssignment> {
    let contents: String = fs::read_to_string("src/days/day4/assets/input.txt")
        .expect("Something went wrong reading the file");

    let elves_assignment: Vec<ElvesAssignment> = contents
        .lines()
        .map(|line| {
            let assignments: Vec<Assignement> = line
                .to_string()
                .split(",")
                .map(|assignement| {
                    let sections: Vec<u32> = assignement.split("-")
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    
                    Assignement {
                        from: sections[0],
                        to: sections[1],
                    }
                })
                .collect::<Vec<Assignement>>();

            ElvesAssignment {
                left: assignments[0],
                right: assignments[1],
            }
        })
        .collect();
    
    elves_assignment
}

pub fn part1() {
    let elves_assignment = get_input();

    let contain_count: u32 = elves_assignment.iter()
        .map(|assignment| {
            if assignment.is_fully_contain() {
                1
            } else {
                0
            }
        })
        .sum();
    
    println!("Day 4 - Part 1: Number of fully contain is {}", contain_count);
}

pub fn part2() {
    let elves_assignment = get_input();

    let contain_count: u32 = elves_assignment.iter()
        .map(|assignment| {
            if assignment.is_overlap() {
                1
            } else {
                0
            }
        })
        .sum();
    
    println!("Day 4 - Part 1: Number of overlap is {}", contain_count);
}