use std::{fs, fmt};
use std::hash::Hash;
use std::collections::HashMap;
use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use petgraph::dot::Dot;

static RECURSION_LIMIT: i32 = 500;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    val: i32,
    loc: [usize; 2]
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H: {}", self.val);
        write!(f, " L: {:?}", self.loc)
    }
}

fn build(curr: &Node, map: &Vec<Vec<i32>>, paths: &mut DiGraphMap::<Node, i32>, vstd: Vec<[usize; 2]>, end: &mut Node, depth: i32) {
    let cr = curr.loc[0] as i32;
    let cc = curr.loc[1] as i32;
    let v = map[curr.loc[0]][curr.loc[1]];
    // if found
    if curr.loc == end.loc {
        *end = *curr;
        return;
    }
    // up
    let dirs: [[i32; 2]; 4] = [[-1,0],[0,1],[1,0],[0,-1]];
    for d in dirs {
        let (i,j): (usize, usize) = ((cr+d[0]) as usize, (cc+d[1]) as usize);
        if i < map.len() as usize && j < map[0].len() as usize && v + 1 >= map[i][j] && !vstd.contains(&[i, j]) {
            println!("Loc: {:?} next loc: {:?} Val: {} next val: {}", curr.loc, [i,j],v,map[i][j]);
            // println!("Val: {} next val: {}",v,map[i][j]);
            let branch = Node {val: map[i][j], loc: [i,j]};
            paths.add_node(branch);
            paths.add_edge(*curr,branch,1);
            let mut vstd_copy = vstd.clone();
            vstd_copy.push(curr.loc);
            let depth_cpy = depth + 1;
            if depth < RECURSION_LIMIT {
                build(&branch, map, paths, vstd_copy, end, depth_cpy);
            }
        }
    }
    return;
}

fn main() {
    let file_path = "src/day_12_inputs_tst2.txt";
    // Build Map
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut map: Vec<Vec<i32>> = vec![];
    let mut s = [0,0];
    let mut e = [0,0];
    let mut line_ct = 0;
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::from_iter(line.chars().map(|x| x as i32));
        for i in 0..row.len() {
            if row[i] == 83 {
                s = [line_ct, i];
                row[i] = 0;
            }
            else if row[i] == 69 {
                e = [line_ct, i];
                row[i] = 25;
            }
            else {row[i] = row[i] - 97;}
        }
        // println!("{row:?}");
        map.push(row);
        line_ct+=1;
    }
    // BF Traversal
    let mut paths = DiGraphMap::<_, i32>::new();
    let start = Node {val: 0, loc: s};
    let mut end = Node {val: 0, loc: e};
    paths.add_node(start);
    let mut visited = vec![];
    visited.push(s);
    println!("Start recursion...");
    build(&start, &map, &mut paths, visited, &mut end, 1);
    // println!("{}", Dot::new(&paths));
    let res = algo::astar(
        &paths,
        start,               // start
        |n| n == end,      // is_goal
        |_| 1, // edge_cost
        |_| 0,           // estimate_cost
    );
    match res {
        Some((cost, path)) => {
            println!("The total cost was {}", cost);
        }
        None => println!("There was no path"),
    }
}