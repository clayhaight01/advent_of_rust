use std::fs;

fn main() {
    let file_path = "C:/Users/User/Documents/GitHub/advent_of_rust/day_6/src/day_6_inputs.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    static MARKER_LENGTH: usize = 4; //can be adjusted for part 1 and part 2

    for line in contents.lines() {
        let mut strt: usize = 0;
        let mut end: usize = MARKER_LENGTH - 1;
        let mut found: bool = false;
        let msg: Vec<char> = line.chars().collect();
        while !found && end < line.len() {
            let mut wndw: Vec<char> = vec![];
            for i in 0..MARKER_LENGTH {wndw.push(msg[strt+i]);}
            let mut unique: bool = true;
            for j in 0..wndw.len() {
                for k in 0..wndw.len() {
                    if j != k && wndw[k] == wndw[j] {
                        unique = false;
                    }
                }
            }
            if unique {found = true}
            strt += 1;
            end += 1;
        }
        if end >= line.len() - 1 {
            end = line.len();
            println!("{}",end);
        }
        println!("{} characters were processed before finding the marker",end)
    }
}