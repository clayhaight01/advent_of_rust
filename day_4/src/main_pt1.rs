use std::fs;
use std::iter::FromIterator;

fn main() {
    // --snip--
    let file_path = "C:/Users/User/Documents/GitHub/advent_of_code/day_4/src/day_4_inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut contains = 0;
    for line in contents.lines() {
        let res_pre = Vec::from_iter(line.split("-").map(String::from));
        let trapped = res_pre[1].split(",");
        let mut res: Vec<u32> = vec![res_pre[0].parse::<u32>().unwrap(),0,0,res_pre[2].parse::<u32>().unwrap()];
        let mut count = 1;
        for num in trapped {
            res[count] = num.parse::<u32>().unwrap();
            count += 1;
        }
        println!("{:?}",res);
        if (res[0] <= res[2] && res[1] >= res[3]) || (res[0] >= res[2] && res[1] <= res[3]) {
            contains += 1;
        }
    }
    println!("{}",contains)
}
