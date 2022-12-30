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

fn vis(v: Vec<u32>) -> u32 {
    if v.len() == 0 {return 0;}
    let mut t = v.iter();
    let th = t.nth(0).unwrap();
    for (i,h) in t.enumerate() {
        if h >= th {return (i+1).try_into().unwrap();}
    }
    (v.len() - 1).try_into().unwrap()
}

fn main() {
    let file_path = "src/day_8_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut forest: Vec<Vec<u32>> = vec![];
    // populate array
    for line in contents.lines() {forest.push(Vec::from_iter(line.chars().map(|c| c.to_digit(10).unwrap())));}
    let mut forest_t = transpose(forest.clone());

    let mut scenic_max = 0;
    for r in 0..forest.len() {
        for c in 0..forest[r].len() {
            println!("[{r},{c}]");
            let sl = vis(Vec::from_iter(forest[r][0..(c+1)].iter().rev().cloned()));
            let sr = vis(Vec::from_iter(forest[r][c..forest[r].len()].iter().cloned()));
            let su = vis(Vec::from_iter(forest_t[c][0..(r+1)].iter().rev().cloned()));
            let sd = vis(Vec::from_iter(forest_t[c][r..forest_t[c].len()].iter().cloned()));
            println!("scores: su {}, sl {}, sr {}, sd {}",su,sl,sr,sd);
            let score = sl * sr * su * sd;
            println!("score: {}",score);
            if score > scenic_max {scenic_max = score;}
        }
    }
    println!("Highest Scenic Score: {scenic_max}");
}
