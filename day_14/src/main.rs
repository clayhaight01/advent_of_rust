use nom::*;
use nom::character::complete::*;
use nom::bytes::complete::*;
use atoi::atoi;

static ASPECT: (usize, usize) = (1000, 200);
static SAND: usize = 500;

fn drop(sec: &Vec<Vec<bool>>, sand: &mut Vec<Vec<bool>>, loc: (usize, usize)) -> bool {
    if !sec[loc.0+1][loc.1] && !sand[loc.0+1][loc.1] {
        drop(sec, sand, (loc.0 + 1, loc.1))
    }
    else if !sec[loc.0+1][loc.1-1] && !sand[loc.0+1][loc.1-1] {
        drop(sec, sand,(loc.0 + 1, loc.1 - 1))
    }
    else if !sec[loc.0+1][loc.1+1] && !sand[loc.0+1][loc.1+1] {
        drop(sec, sand, (loc.0 + 1, loc.1 + 1))
    }
    else if loc.0 == 0 && loc.1 == SAND {
        return false;
    }
    else {
        sand[loc.0][loc.1] = true;
        return true;
    }
}

fn main() {
    // Build Wall
    let mut x_bound = (ASPECT.0, 0);
    let mut y_bound = (0, 0);
    let mut sec = vec![vec![false; ASPECT.0]; ASPECT.1];
    let mut sand = vec![vec![false; ASPECT.0]; ASPECT.1];
    let walls: Vec<Vec<(usize, usize)>> = include_str!("day_14_inputs.txt").split("\n").map(|a| list(a.as_bytes()).unwrap().1).collect();
    for wall in walls {
        for l in 1..wall.len() {
            let mut start = wall[l-1];
            let mut end = wall[l];
            if start.0 > end.0 {
                let temp = end.clone();
                end.0 = start.0;
                start.0 = temp.0;
            }
            if start.1 > end.1 {
                let temp = end.clone();
                end.1 = start.1;
                start.1 = temp.1;
            }
            if start.0 < x_bound.0 {x_bound.0 = start.0}
            if end.0 > x_bound.1 {x_bound.1 = end.0}
            if end.1 > y_bound.1 {y_bound.1 = end.1}
            if start.0 == end.0 {for i in start.1..=end.1 {sec[i][start.0] = true}}
            else {for i in start.0..=end.0 {sec[start.1][i] = true}}
        }
    }
    for i in 0..ASPECT.0 {sec[y_bound.1 + 2][i] = true}
    let mut units = 1;
    while drop(&sec, &mut sand, (0,SAND)) {units += 1;}
   
    // Fancy Print
    for i in y_bound.0..=y_bound.1 {
        let mut row: String = String::new();
        for j in x_bound.0..=x_bound.1 {
            if i == 0 && j == SAND {row.push('+')}
            else if sec[i][j] {row.push('#')}
            else if sand[i][j] {row.push('o')}
            else {row.push(' ')}
        }
        println!("{i} {row}");
    }
    println!("Units: {units}")
}

// Nom
named!(list(&[u8]) -> Vec<(usize, usize)>, separated_list0!(tag(" -> "), coord));
named!(coord(&[u8]) -> (usize, usize), separated_pair!(map_opt!(digit1, atoi), char!(','), map_opt!(digit1, atoi)));