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
    // recursive traversal function
    //fn sum_under<'a>(&self, mut sum: &'a i32) -> (i32, i32) {
    fn sum_under<'a>(&self, sum: &'a mut i32) -> (i32, &'a mut i32) {
        //add files in current directory
        let mut local_sum: i32 = *self.size.borrow();

        //add sizes of directories in current directory
        for child in self.children.borrow().iter() {
            let recurse_sum = child.sum_under(sum);
            local_sum += recurse_sum.0;
        }
        let mut acpt_sum = sum;
        // if this sum is less than 100000, return it in a secondary value
        if local_sum < 100000 {
            *acpt_sum += local_sum;
            println!("{acpt_sum}");
        }
        (local_sum, acpt_sum)
    }
}
// go through each ls list, if directory, recursively explore
// if file, add value to parent directory total size
// link all while doing it
fn main() {
    let file_path = "src/day_7_inputs_tst.txt";

    let root = Rc::new(Node {
        size: RefCell::new(0),
        filename: String::from("/"),
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new())
    });

    let mut curr_node = Rc::clone(&root);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let mut segs = line.split_whitespace();
        let first = segs.nth(0).unwrap();
        let second = segs.nth(0).unwrap();
        if first == "$"{ // user prompt
            if second == "cd" { // cd
                let third = segs.nth(0).unwrap();
                if third == ".."{ //out
                    let temp_node = Rc::clone(&curr_node);
                    curr_node = temp_node.parent.borrow().upgrade().unwrap();
                }
                else { //in
                    let new_node = Rc::new(Node {
                        size: RefCell::new(0),
                        filename: String::from(third),
                        children: RefCell::new(Vec::new()),
                        parent: RefCell::new(Weak::new())
                    });
                    new_node.add_parent(Rc::downgrade(&curr_node));
                    curr_node.add_child(Rc::clone(&new_node));
                    curr_node = new_node;
                }
            }
        }
        else {
            if let Result::Ok(file_size) = first.parse::<i32>() {
                curr_node.add(file_size);
                //println!("value: {}", curr_node.size.borrow());
            }
        }
    }
    let mut sum: i32 = 0;
    let ans: (i32, &mut i32) = root.sum_under(&mut sum);
    println!("total sum: {}",ans.1);
}
