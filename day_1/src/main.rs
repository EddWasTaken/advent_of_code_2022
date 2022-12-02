use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    
    let mut tmp_calories = 0;
    let calories_lines = read_lines("input.txt");
    let mut elf_count = 0;
    let mut elf_calories: HashMap<i32, i32> = HashMap::new();
    let mut highest_elf = vec![0, 0, 0];
    let mut highest_calories: Vec<i32> = vec![0, 0, 0];
    for line in calories_lines {
        if line.is_empty() {
            elf_count += 1;
            if tmp_calories > highest_calories[2] && tmp_calories > highest_calories[1] && tmp_calories > highest_calories[0] {
                highest_calories[2] = highest_calories[1];
                highest_calories[1] = highest_calories[0];
                highest_calories[0] = tmp_calories;
                highest_elf[2] = highest_elf[1];
                highest_elf[1] = highest_elf[0];
                highest_elf[0] = elf_count;
            }
            elf_calories.insert(elf_count, tmp_calories);
            tmp_calories = 0;

        } else {
            tmp_calories += line.parse::<i32>().unwrap();
        }

    }
    println!("Highest cal: {}, {}, {}", highest_calories[0], highest_calories[1], highest_calories[2]);
    println!("The elves with the highest calories are {}, {} and {}, with a total of {} calories!", highest_elf[0], highest_elf[1], highest_elf[2], highest_calories.iter().fold(0, |acc, x| acc + x ));
    Ok(())
}

fn read_lines(filename: &str) -> Vec<String> {
    let input = File::open(filename).expect("No such file");
    let buf_reader = BufReader::new(input);
    buf_reader.lines()
              .map(|l| l.expect("Could not parse line"))
              .collect()
}