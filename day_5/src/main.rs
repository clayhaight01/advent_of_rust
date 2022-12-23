use std::fs;

fn main() {
    static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
    ];

    let file_path = "C:/Users/User/Documents/GitHub/advent_of_rust/day_5/src/day_5_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut start = 0;
    let mut line_ct = 0;
    let mut found: bool = false;
    let mut stacks: Vec<Vec<u32>>  = vec![vec![]; 9];
    for line in contents.lines() {
        if line != "" && !found {} else {found=true;}
        if !found {
            start+=1;
            let mut int = Vec::from_iter(line.chars());
            let mut char_ct: u32 = 0;
            for i in int {
                if i != ' ' && i != '[' && i != ']' {
                    let stack_ct: u32 = (char_ct + 3)/4
                    stacks[stack_ct].push(i)
                }
                char_ct += 1;
            }
        }
        line_ct+=1;
    }
}
