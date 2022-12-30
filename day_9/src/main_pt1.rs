use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "src/day_9_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (mut h, mut t): ((i32,i32),(i32,i32)) = Default::default();
    let mut s: HashSet<(i32, i32)> = HashSet::new();
    s.insert((0,0));
    for line in contents.lines() {
        let mut cmd_str = line.split(' ');
        let d = cmd_str.nth(0).unwrap();
        let l = cmd_str.nth(0).unwrap().parse::<i32>().unwrap();
        let cmd = match d {
            "U" => [0,1,l],
            "D" => [0,-1,l],
            "L" => [-1,0,l],
            _ => [1,0,l],
        };
        for _ in 0..cmd[2] {
            h = (h.0 + cmd[0], h.1 + cmd[1]);
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                t = (h.0 - cmd[0], h.1 - cmd[1]);
                s.insert(t);
            }
        }
    }
    println!("{}",s.len());
}
