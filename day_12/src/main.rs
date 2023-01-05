use std::{fs, fmt};
use std::hash::Hash;
use petgraph::graphmap::DiGraphMap;
use petgraph::dot::Dot;

static RECURSION_LIMIT: i32 = 7000;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    id: i32,
    dep: i32,
    val: i32,
    loc: [usize; 2],
    dir: char,
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "D: {}", self.dep);
        write!(f, "H: {}", self.val);
        write!(f, " L: {:?}", self.loc)
    }
}

fn build(curr: &Node, map: &Vec<Vec<i32>>, paths: &mut DiGraphMap::<Node, i32>, vstd: Vec<[usize; 2]>, end: [usize; 2], depth: i32, id: &mut i32, depths: &mut Vec<i32>) { //add depth value to return?
    let cr = curr.loc[0] as i32;
    let cc = curr.loc[1] as i32;
    let v = map[curr.loc[0]][curr.loc[1]];
    // if found
    if curr.loc == end {
        depths.push(depth);
        return;
    }
    // up
    let dirs: [[i32; 2]; 4] = [[-1,0],[0,1],[1,0],[0,-1]];
    for d in dirs {
        // println!("Directions:");
        // println!("{} >= 0",cr+d[0]);
        // println!("{} >= 0",cc+d[1]);
        // println!("{} < {}",cr+d[0],map.len());
        // println!("{} < {}",cc+d[1],map[0].len());
        let (i,j): (usize, usize) = ((cr+d[0]) as usize, (cc+d[1]) as usize);
        if i < map.len() as usize && j < map[0].len() as usize && 0 <= map[i][j] - v && map[i][j] - v < 2 &&  !vstd.contains(&[i, j]) {
            println!("Loc: {:?} next loc: {}",curr.loc,map[i][j]);
            // println!("Val: {} next val: {}",v,map[i][j]);
            let branch = Node {id: *id, dep: depth, val: map[i][j], loc: [i,j], dir: 'O'};
            paths.add_node(branch);
            paths.add_edge(*curr,branch,1);
            let mut vstd_copy = vstd.clone();
            vstd_copy.push(curr.loc);
            let mut depth_copy = depth.clone();
            depth_copy += 1;
            *id += 1;
            if depth < RECURSION_LIMIT {
                build(&branch, map, paths, vstd_copy, end, depth_copy, id, depths);
            }
            else {return;}
        }
        // else {return;}
    }
    
}

fn main() {
    let file_path = "src/day_12_inputs.txt";

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
        println!("{row:?}");
        map.push(row);
        line_ct+=1;
    }
    // BF Traversal
    let mut paths = DiGraphMap::<_, i32>::new();
    let start = Node {id: 0, dep: 0, val: 0, loc: s, dir: 'O'};
    paths.add_node(start);
    let mut visited = vec![];
    visited.push(s);
    let mut depths: Vec<i32> = vec![];
    let mut id: i32 = 1;
    build(&start, &map, &mut paths, visited, e, 0, &mut id, &mut depths);
    // println!("{}", Dot::new(&paths));
    depths.sort();
    println!("{:?}",depths);
}