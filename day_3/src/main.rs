use std::fs;

fn main() {
    // --snip--
    let file_path = "C:/Users/User/Documents/GitHub/advent_of_rust/day_3/src/day_3_inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let mut lc = 0;

    let mut line_count = 0;
    for _line in contents.lines() {
        line_count += 1;
    }

    while lc < (line_count - 2) {
        let ruck1: Vec<char> = contents.lines().nth(lc).unwrap().chars().collect();
        let ruck2: Vec<char> = contents.lines().nth(lc+1).unwrap().chars().collect();
        let ruck3: Vec<char> = contents.lines().nth(lc+2).unwrap().chars().collect();
        let mut ruck_num1: Vec<u32> = vec![0; ruck1.len()];
        let mut ruck_num2: Vec<u32> = vec![0; ruck2.len()];
        let mut ruck_num3: Vec<u32> = vec![0; ruck3.len()];

        let mut count = 0;
        for item in ruck1 {
            if (item as u32) > 64 && (item as u32) < 91 {
                ruck_num1[count] = item as u32 - 38;
            }
            else {
                ruck_num1[count] = item as u32 - 96;
            }
            count += 1;
        }
        let mut count = 0;
        for item in ruck2 {
            if (item as u32) > 64 && (item as u32) < 91 {
                ruck_num2[count] = item as u32 - 38;
            }
            else {
                ruck_num2[count] = item as u32 - 96;
            }
            count += 1;
        }
        let mut count = 0;
        for item in ruck3 {
            if (item as u32) > 64 && (item as u32) < 91 {
                ruck_num3[count] = item as u32 - 38;
            }
            else {
                ruck_num3[count] = item as u32 - 96;
            }
            count += 1;
        }

        for item in ruck_num1 {
            if ruck_num2.contains(&item) && ruck_num3.contains(&item) {
                sum += item;
                break;
            }
        }
        lc += 3;
    }
    println!("{}",sum);
}
