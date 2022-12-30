use std::fs;

fn main() {
    let file_path = "src/day_10_inputs.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut cycle = 0;
    let mut load_cycle = 0;
    let mut load_val = 0;
    let mut x = 1;
    let mut sig = 0;
    for line in contents.lines() {
        let mut cmd_str = line.split(' ');
        let cmd = cmd_str.nth(0).unwrap();
        if cmd == "addx" { //add
            for _ in 0..2 {
                cycle += 1;
                if (cycle+20)%40 == 0 && cycle <= 220 {
                    println!("*********************************************************");
                    sig += cycle * x;
                }
                
                println!("Cycle: {}",cycle);
                println!("adding...");
                println!("X: {}",x);
            }
            x+=cmd_str.nth(0).unwrap().parse::<i32>().unwrap();
        }
        else { //noop
            cycle += 1;
            if (cycle+20)%40 == 0 && cycle <= 220 {
                println!("*********************************************************");
                sig += cycle * x;
            }
            println!("Cycle: {}",cycle);
            println!("nooping...");
            println!("X: {}",x);
        }
    }
    println!("Total Signal Strength: {sig}");
}