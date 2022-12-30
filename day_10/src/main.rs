use std::fs;

fn main() {
    let file_path = "src/day_10_inputs.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut sig = 0;
    let mut crt: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    let mut pos: [i32; 2] = [0,0];
    for line in contents.lines() {
        let mut cmd_str = line.split(' ');
        let cmd = cmd_str.nth(0).unwrap();
        if cmd == "addx" { //add
            for _ in 0..2 {
                cycle += 1;
                pos[0] = cycle - pos[1]*40-1;
                println!("Pos: [{},{}]",pos[0],pos[1]);
                if pos[0].abs_diff(x) < 2 {
                    crt[pos[1] as usize][pos[0] as usize] = '#';
                }
                else {
                    crt[pos[1] as usize][pos[0] as usize] = '.';
                }
                if cycle%40 == 0 && cycle <= 220 {
                    sig += cycle * x;
                    pos[1] += 1; //add row
                }
                println!("Cycle: {}",cycle);
                println!("adding...");
                println!("X: {}",x);
            }
            x+=cmd_str.nth(0).unwrap().parse::<i32>().unwrap();
        }
        else { //noop
            cycle += 1;
            pos[0] = cycle - pos[1]*40-1;
            println!("Pos: [{},{}]",pos[0],pos[1]);
            if pos[0].abs_diff(x) < 2 {
                crt[pos[1] as usize][pos[0] as usize] = '#';
            }
            else {
                crt[pos[1] as usize][pos[0] as usize] = '.';
            }
            if cycle%40 == 0 && cycle <= 220 {
                sig += cycle * x;
                pos[1] += 1; //add row
            }
            println!("Cycle: {}",cycle);
            println!("nooping...");
            println!("X: {}",x);
        }
        let mut crt_str: [String; 6] = Default::default();
        for r in 0..crt.len() {crt_str[r] = crt[r].iter().collect();}
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",crt_str[0],crt_str[1],crt_str[2],crt_str[3],crt_str[4],crt_str[5])
    }
    println!("Total Signal Strength: {sig}");
}