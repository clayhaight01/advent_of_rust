use std::fs;

fn main() {
    // --snip--
    let file_path = "src/day_3_inputs.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let ruck: Vec<char> = line.chars().collect();
        let half = ruck.len()/2 as usize;
        let mut ruck_num_1: Vec<u32> = vec![0; half];
        let mut ruck_num_2: Vec<u32> = vec![0; half];
        let mut count = 0;
        
        for item in ruck {
            if (item as u32) > 64 && (item as u32) < 91 && count < half {
                ruck_num_1[count] = item as u32 - 38;
            }
            else if (item as u32) > 64 && (item as u32) < 91 {
                ruck_num_2[count-half] = item as u32 - 38;
            }
            else if count < half {
                ruck_num_1[count] = item as u32 - 96;
            }
            else {
                ruck_num_2[count-half] = item as u32 - 96;
            }
            count += 1;
        }
        println!("ruck 1: {:?}",ruck_num_1);
        println!("ruck 2: {:?}",ruck_num_2);
        for item in ruck_num_1 {
            if ruck_num_2.contains(&item) {
                sum += item;
                println!("item: {}",item);
                break;
            }
        }
    }
    println!("{}",sum);
}
