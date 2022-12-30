use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "src/day_9_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut rope = [[0i32;2]; 10];
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
            rope[0][0] += cmd[0];
            rope[0][1] += cmd[1];
            for k in 0..(rope.len()-1) {
                if rope[k][0].abs_diff(rope[k+1][0]) > 1 || rope[k][1].abs_diff(rope[k+1][1]) > 1 {
                    rope[k+1][0] = rope[k+1][0] + (rope[k][0] - rope[k+1][0]).signum();
                    rope[k+1][1] = rope[k+1][1] + (rope[k][1] - rope[k+1][1]).signum();
                }
            }
            s.insert((rope[rope.len()-1][0],rope[rope.len()-1][1]));
        }
    }
    println!("Tail visited: {} spots",s.len());
}
