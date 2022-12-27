use std::fs;

fn main() {
    let file_path = "src/day_1_inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut v: Vec<i32> = Vec::new();
    let mut loc_sum: i32 = 0;
    let mut tot_sum: i32 = 0;
    for line in contents.lines() {
        if line == "" {
            v.push(loc_sum);
            loc_sum = 0;
        }
        else {
            let line_int = line.parse::<i32>().unwrap();
            loc_sum = loc_sum + line_int;
        }
    }
    v.sort();
    for i in v.len()-3..v.len() {
        tot_sum = tot_sum + v[i];
    }
    println!("Most Calories: {}", tot_sum);
}
