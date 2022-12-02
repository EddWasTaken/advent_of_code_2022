use std::{
        io::{
            BufReader,
            BufRead,
    },
        fs::{
            File,
        }
};
    /*
    Rock        = 1
    Paper       = 2
    Scissors    = 3
    Losing      = 0
    Drawing     = 3
    Winning     = 6
    */

    /* 
    Challenge 2:
    A, X = Rock
    B, Y = Paper
    C, Z = Scissors
    */
    /*
    Challenge 1:
    X = lose
    Y = Draw
    Z = Win
    */

fn main() {
    let strategy_lines = get_lines("input.txt");
    let mut score = 0;
    for strategy in &strategy_lines {
        match strategy.as_str() {
            "A X" => score += 4,
            "A Y" => score += 8,
            "A Z" => score += 3,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 7,
            "C Y" => score += 2,
            "C Z" => score += 6,
            _ => panic!("Error, couldn't parse strategy")
        };
    }
    println!("Total score using the original strategy: {}", score);
    let mut score2 =0;
    for strategy in strategy_lines {
        match strategy.as_str() {
            "A X" => score2 += 3,
            "A Y" => score2 += 4,
            "A Z" => score2 += 8,
            "B X" => score2 += 1,
            "B Y" => score2 += 5,
            "B Z" => score2 += 9,
            "C X" => score2 += 2,
            "C Y" => score2 += 6,
            "C Z" => score2 += 7,
            _ => panic!("Error, couldn't parse strategy")
        };
    }
    println!("Total score using the secret strategy: {}", score2);

}

fn get_lines(filename: &str) -> Vec<String> {

    let file = File::open(filename).expect("No such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}