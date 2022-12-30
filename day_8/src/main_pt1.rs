use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn main() {
    let file_path = "src/day_8_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut forest: Vec<Vec<u32>> = vec![];
    // populate array
    for line in contents.lines() {forest.push(Vec::from_iter(line.chars().map(|c| c.to_digit(10).unwrap())));}

    println!("BEFORE");
    println!("{:?}",forest);
    let mut vis = 0;
    let mut seen: Vec<Vec<bool>> = vec![vec![false; forest[0].len()]; forest.len()];
    for i in 0..forest.len() {
        let mut lmax = 0;
        println!("LEFT");
        for left in 0..forest[i].len() {
            if forest[i][left] > lmax || left == 0 {
                lmax = forest[i][left];
                if !seen[i][left]  {
                    vis += 1;
                    seen[i][left] = true;
                    println!("Seen at [{i},{left}]");
                }
            }
            
        }
        let mut rmax = 0;
        println!("RIGHT");
        for right in (0..forest[i].len()).rev() {
            if forest[i][right] > rmax || right == forest[i].len() - 1 {
                rmax = forest[i][right];
                if !seen[i][right] {
                    vis += 1;
                    seen[i][right] = true;
                    println!("Seen at [{i},{right}]");
                }
            }
        }
    }
    let mut forest_t = transpose(forest);
    println!("transposed: {:?}",forest_t);
    let mut seen_t = transpose(seen);
    for i in 0..forest_t.len() {
        let mut lmax = 0;
        println!("LEFT");
        for left in 0..forest_t[i].len() {
            if forest_t[i][left] > lmax || left == 0 {
                lmax = forest_t[i][left];
                if !seen_t[i][left]  {
                    vis += 1;
                    seen_t[i][left] = true;
                    println!("Seen at [{i},{left}]");
                }
            }
            
        }
        let mut rmax = 0;
        println!("RIGHT");
        for right in (0..forest_t[i].len()).rev() {
            if forest_t[i][right] > rmax || right == forest_t[i].len() - 1 {
                rmax = forest_t[i][right];
                if !seen_t[i][right] {
                    vis += 1;
                    seen_t[i][right] = true;
                    println!("Seen at [{i},{right}]");
                }
            }
        }
    }
    forest = transpose(forest_t);
    seen = transpose(seen_t);
    println!("AFTER");
    println!("{:?}",forest);
    println!("SEEN");
    println!("{:?}",seen);
    println!("visible in row: {vis}");

}
