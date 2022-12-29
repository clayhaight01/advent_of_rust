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

    println!("before: {:?}",forest);
    let mut vis = 0;
    for i in 0..forest.len() {
        let mut lmax = 0;
        for left in 0..forest[i].len() {
            if forest[i][left] > lmax {
                lmax = forest[i][left];
                vis += 1;
                forest[i][left] = 0;
            }
        }
        let mut rmax = 0;
        for right in (0..forest[i].len()).rev() {
            if forest[i][right] > rmax {
                rmax = forest[i][right];
                vis += 1;
                forest[i][right] = 0;
            }
        }
    }
    let mut forest_t = transpose(forest);
    for i in 0..forest_t.len() {
        let mut lmax = 0;
        for left in 0..forest_t[i].len() {
            if forest_t[i][left] > lmax {
                lmax = forest_t[i][left];
                vis += 1;
                forest_t[i][left] = 0;
            }
        }
        let mut rmax = 0;
        for right in (0..forest_t[i].len()).rev() {
            if forest_t[i][right] > rmax {
                rmax = forest_t[i][right];
                vis += 1;
                forest_t[i][right] = 0;
            }
        }
    }
    forest = transpose(forest_t);
    println!("after: {:?}",forest);
    println!("visible in row: {vis}");

}
