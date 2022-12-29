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
    let file_path = "src/day_8_inputs_tst.txt";

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
        for left in 0..forest[i].len() {
            if (forest[i][left] > lmax || left == 0) && !seen[i][left] {
                lmax = forest[i][left];
                vis += 1;
                seen[i][left] = true;
            }
        }
        let mut rmax = 0;
        for right in (0..forest[i].len()).rev() {
            if (forest[i][right] > rmax || right == forest[i].len() - 1) && !seen[i][right] {
                rmax = forest[i][right];
                vis += 1;
                seen[i][right] = true;
            }
        }
    }
    // let mut forest_t = transpose(forest);
    // let mut seen_t = transpose(seen);
    // for i in 0..forest_t.len() {
    //     let mut lmax = 0;
    //     for left in 0..forest_t[i].len() {
    //         if (forest_t[i][left] > lmax || left == 0) && !seen_t[i][left] {
    //             lmax = forest_t[i][left];
    //             vis += 1;
    //             seen_t[i][left] = true;
    //         }
    //     }
    //     let mut rmax = 0;
    //     for right in (0..forest_t[i].len()).rev() {
    //         if (forest_t[i][right] > rmax || right == forest_t[i].len()-1) && !seen_t[i][right] {
    //             rmax = forest_t[i][right];
    //             vis += 1;
    //             seen_t[i][right] = true;
    //         }
    //     }
    // }
    // forest = transpose(forest_t);
    // seen = transpose(seen_t);
    println!("AFTER");
    println!("{:?}",forest);
    println!("SEEN");
    println!("{:?}",seen);
    println!("visible in row: {vis}");

}
