use std::fs;

#[derive(Debug)]
pub struct Stacks {
    data: Vec<Vec<char>>
}

#[derive(Debug)]
pub struct Rearrangement {
    quantity: u32,
    from: usize,
    to: usize
}

impl Stacks {
    pub fn rearange(&mut self, r: Rearrangement) {
        let q = r.quantity;
        let from = r.from - 1;
        let to = r.to - 1;

        for _ in 0..q {
            match self.data[from].pop() {
                Some(c) => self.data[to].push(c),
                None => println!("Error in rearangement!"),
            };
        }
    }

    pub fn rearange_reserve_order(&mut self, r: Rearrangement) {
        let q = r.quantity;
        let from = r.from - 1;
        let to = r.to - 1;

        let len = self.data[from].len();
        for _ in 0..q {
            let c = self.data[from].remove(len - q as usize);
            self.data[to].push(c);
        }
    }
}

pub fn get_stacks() -> Stacks {
    let contents: String = fs::read_to_string("src/days/day5/assets/input.txt")
    .expect("Something went wrong reading the file");
    let stacks_info: Vec<Vec<char>> = contents.lines()
        .take_while(|line| !line.is_empty())
        .map(
            |line| {
                let mut chars= vec![];
                for i in 0..line.len() {
                    if i % 4 == 1 {
                        match line.chars().nth(i) {
                            Some(c) => chars.push(c),
                            _ => println!("Char not found!"),
                        }
                    }
                }
                chars
            }
        )
        .collect();
    
    let mut stacks: Vec<Vec<char>> = vec![];
    for j in 0..stacks_info[0].len() {
        let mut stack: Vec<char> = vec![];
        for i in (0..stacks_info.len() - 1).rev() {
            if stacks_info[i][j] != ' ' {
                stack.push(stacks_info[i][j]);
            }
        }
        stacks.push(stack);
    }
    Stacks {
        data: stacks
    }
}

pub fn get_rearangement() -> Vec<Rearrangement> {
    let contents: String = fs::read_to_string("src/days/day5/assets/input.txt")
    .expect("Something went wrong reading the file");
    let rearangements: Vec<Rearrangement> = contents.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(
            |s| {

                let quantity: u32 = match s.split(" ").nth(1).unwrap().parse() {
                    Ok(d) => d,
                    _ => 0
                };
                let from: usize = match s.split(" ").nth(3).unwrap().parse() {
                    Ok(d) => d,
                    _ => 0
                };
                let to: usize = match s.split(" ").nth(5).unwrap().parse() {
                    Ok(d) => d,
                    _ => 0
                };
                Rearrangement {
                    quantity,
                    from,
                    to
                }
            }
        )
        .collect();
    rearangements
}

pub fn part1() {
    let mut stacks = get_stacks();
    let rearangements = get_rearangement();
    for r in rearangements.into_iter() {
        stacks.rearange(r);
    }
    let top_chars : String = stacks.data.into_iter()
        .map(
            |chars|
            match chars.last() {
                Some(c) => *c,
                None => ' ',
            }
        )
        .filter(|c| *c != ' ')
        .collect();
    println!("Day 5 - Part 1: Top chars is {}", top_chars);
}

pub fn part2() {
    let mut stacks = get_stacks();
    let rearangements = get_rearangement();
    for r in rearangements.into_iter() {
        stacks.rearange_reserve_order(r);
    }
    let top_chars : String = stacks.data.into_iter()
        .map(
            |chars|
            match chars.last() {
                Some(c) => *c,
                None => ' ',
            }
        )
        .filter(|c| *c != ' ')
        .collect();
    println!("Day 5 - Part 2: Top chars is {}", top_chars);
}