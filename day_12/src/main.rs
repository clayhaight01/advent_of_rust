use std::{fs, fmt};
use std::hash::Hash;
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::Dot;

static RECURSION_LIMIT: i32 = 10;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    loc: [usize; 2],
    dir: char,
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loc: {:?}", self.loc)
    }
}

fn build(curr: &Node, map: &Vec<Vec<u32>>, paths: &mut UnGraphMap::<Node, i32>, start: [usize; 2], end: [usize; 2], depth: &mut i32) { //add depth value to return?
    let cr = curr.loc[0] as i32;
    let cc = curr.loc[1] as i32;
    let v = map[curr.loc[0]][curr.loc[1]];
    // if found
    if curr.loc == end {return;}
    // up
    let dirs: [[i32; 2]; 4] = [[-1,0],[0,1],[1,0],[0,-1]];
    for d in dirs {
        println!("Directions:");
        println!("{} >= 0",cr+d[0]);
        println!("{} >= 0",cc+d[1]);
        println!("{} < {}",cr+d[0],map.len());
        println!("{} < {}",cc+d[1],map[0].len());

        let (i,j): (usize,usize) = ((cr+d[0]) as usize, (cc+d[1]) as usize);
        if cr+d[0] >= 0 && cc+d[1] >= 0 && cr+d[0] < map.len() as i32 && cc+d[1] < map[0].len() as i32 && (map[(cr+d[0]) as usize][(cc+d[1]) as usize] + 1 >= v) {
            println!("enter");
            let mut branch = Node {loc: [i,j], dir: 'O'};
            paths.add_node(branch);
            paths.add_edge(*curr,branch,1);
            *depth += 1;
            if *depth < RECURSION_LIMIT {build(&branch, map, paths, start, end, depth);}
            else {return;}
        }
    }
    
}

fn main() {
    let file_path = "src/day_12_inputs_tst.txt";

    // Build Map
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut map: Vec<Vec<u32>> = vec![];
    let mut s = [0,0];
    let mut e = [0,0];
    let mut line_ct = 0;
    for line in contents.lines() {
        let mut row: Vec<u32> = Vec::from_iter(line.chars().map(|x| x as u32));
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
    let mut paths = UnGraphMap::<_, i32>::new();
    let start = Node {
        loc: s,
        dir: 'O',
    };
    paths.add_node(start);
    let mut depth = 0;
    build(&start, &map, &mut paths, s, e, &mut depth);
    println!("{}", Dot::new(&paths));
}