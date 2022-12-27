use std::fs;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Node {
    size: RefCell<i32>,
    filename: String,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn add_parent(&self, node: Weak<Node>){
        *self.parent.borrow_mut() = node;
    }
    fn add_child(&self, node: Rc<Node>){
        self.children.borrow_mut().push(node)
    }
    fn add(&self, add: i32){
        *self.size.borrow_mut() += add;
    }
}
// go through each ls list, if directory, recursively explore
// if file, add value to parent directory total size
// link all while doing it


fn main() {
    let file_path = "src/day_7_inputs.txt";

    let root = Rc::new(Node {
        size: RefCell::new(0),
        filename: String::from("/"),
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new())
    });
    let mut currentNode = Rc::clone(&root);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let mut segs = line.split_whitespace();
        let mut first = segs.nth(0).unwrap();
        let mut second = segs.nth(0).unwrap();
        if first == "$"{ // user prompt
            if second == "cd" { // cd
                if segs.nth(0).unwrap() == ".."{ //out
                    //println!("out");
                }
                else { //in
                    //println!("{}",last_seg);
                }
            }
        }
        else {
            if let Result::Ok(file_size) = first.parse::<i32>() {
                currentNode.add(file_size);
                println!("value: {}", currentNode.size.borrow());
            }
        }
    }
}
