use std::fs;

fn main() {
    static CHAR_IGNORE: [char; 3] = [' ', '[', ']',];
    static CHAR_NUMS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0',];
    static STACK_COUNT: usize = 9;

    let file_path = "src/day_5_inputs_tst.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut start = 0;
    let mut line_ct = 0;
    let mut found: bool = false;
    let mut stacks: Vec<Vec<char>>  = vec![vec![]; STACK_COUNT];
    for line in contents.lines() {
        if line != "" && !found {} else {found=true;}
        if !found {
            start+=1;
            let int = Vec::from_iter(line.chars());
            let mut char_ct: u32 = 0;
            for i in int {
                if !CHAR_IGNORE.contains(&i) && !CHAR_NUMS.contains(&i) {
                    let stack_ct: usize = ((char_ct + 3)/4 - 1).try_into().unwrap();
                    stacks[stack_ct].push(i);
                }
                char_ct += 1;
            }
        }
        else if line_ct == start {
            for s in 0..STACK_COUNT {stacks[s].reverse();}
        }
        else {
            let mut ins_str = Vec::from_iter(line.split(' ').map(String::from));
            let mut ins: Vec<usize> = vec![];
            for t in 0..3 {ins_str.remove(t);}
            // theres no way this is the best way to convert a vec of strings to ints but im on a plane rn
            for i in ins_str {
                ins.push(i.parse::<usize>().unwrap());
            }
            //if stacks[ins[1]-1].len()<ins[0] {ins[0]=stacks[ins[1]-1].len()}
            for m in (stacks[ins[1]-1].len()-ins[0])..stacks[ins[1]-1].len() { //ins[0] is the quantity to move
                let to_move = stacks[ins[1]-1][m];
                stacks[ins[2]-1].push(to_move); //add to new stack
            }
            println!("Stack Number {}",line_ct-start);
            for s in 0..STACK_COUNT {
                println!("{:?}",stacks[s]);
            }
        }
        line_ct+=1;
    }
}

// Final answer: D H B J Q J C C W