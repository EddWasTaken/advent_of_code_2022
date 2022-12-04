use std::{
    io::{
        BufReader,
        BufRead,
    },
    fs::{
        File,
    },
};
fn main() {
    let rucksacks = get_lines("input.txt");
    part1(&rucksacks);
    part2(&rucksacks);
}


fn get_lines(filename: &str) -> Vec<String> {

    let file = File::open(filename).expect("No such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part1(rucksacks: &Vec<String>) {
    let mut sum_prio: u32 = 0;
    for rucksack in rucksacks {
        let (left, right) = rucksack.split_at(rucksack.len()/2);
        let left_chars = left.chars().collect::<Vec<char>>();
        let right_chars = right.chars().collect::<Vec<char>>();
        for i in 0..left_chars.len() {
            let priority: u32;
            if left_chars.contains(&right_chars[i]) {
                if right_chars[i].is_uppercase() {
                    priority = right_chars[i] as u32 - 38;
                } else {
                    priority = right_chars[i] as u32 - 96;
                }
                sum_prio += priority;
                break;
                
            }

        }
    }
    println!("Sum of all priorities: {}", sum_prio);
}

fn part2(rucksacks: &Vec<String>) {
    let mut sum_prio = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let items: Vec<char> = rucksacks[i+2].chars().collect();
        for j in 0..items.len() {
            let priority: u32;
            if rucksacks[i].contains(items[j]) {
                if rucksacks[i+1].contains(items[j]){
                    if items[j].is_uppercase() {
                        priority = items[j] as u32 - 38;
                    } else {
                        priority = items[j] as u32 - 96;
                    }
                    sum_prio += priority;
                    break;
                }
            }
        }   
    }
    println!("Sum of all priorities: {}", sum_prio);
}