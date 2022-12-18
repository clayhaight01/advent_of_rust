use std::fs;

fn main() {
    let file_path = "C:/Users/User/Documents/GitHub/advent_of_rust/day_2/src/day_2_inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut score = 0;
    let C_char: char = 'C';
    for line in contents.lines() {
        let opp = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();
        let outcome = (outcome as u32 - 88) * 3;
        let opp = opp as u32 - 65;
        
        println!("{}",outcome);
        score += outcome;
        if outcome == 0 {
            if opp == 0 {
                score += 3;
            }
            else {
                score += opp;
            } 
        }
        else if outcome == 3 {
            score += opp + 1
        }
        else if outcome == 6 {
            if opp == 2 {
                score += 1;
            }
            else {
                score += opp + 2;
            }
        }
    }
    println!("{}",score);
}
