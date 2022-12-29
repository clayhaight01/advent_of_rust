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
    fn total<'a>(&self, sum: &'a mut i32) -> &'a mut i32 {    

        *sum += *self.size.borrow();
    
        for child in self.children.borrow().iter() {
          child.total(sum);
        }
        sum
      }
    // recursive traversal function
    fn sum_under<'a>(&self, sum: &'a mut i32, req: i32, candidates: &'a mut Vec<i32>) -> (i32, i32, &'a mut Vec<i32>) {
        //add files in current directory
        let mut local_sum: i32 = *self.size.borrow();

        //add sizes of directories in current directory
        for child in self.children.borrow().iter() {
            let recurse_sum = child.sum_under(sum, req, candidates);
            local_sum += recurse_sum.0;
        }
        let acpt_sum = sum;
        // if this sum is more than required, push into vector
        if local_sum > req {
            candidates.push(local_sum);
        }
        (local_sum, req, candidates)
    }
}
// go through each ls list, if directory, recursively explore
// if file, add value to parent directory total size
// link all while doing it
fn main() {
    static TOTAL_SPACE: i32 = 70000000;
    static REQ_SPACE: i32 = 30000000;

    let file_path = "src/day_7_inputs.txt";

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
    let mut sum1: i32 = 0;
    let used = *root.total(&mut sum1);
    let space = TOTAL_SPACE - used;
    let to_delete = REQ_SPACE - space;
    println!("used space: {used}");
    println!("free space: {space}");
    println!("amount to delete: {to_delete}");
    let mut candidates: Vec<i32> = vec![];
    let mut sum2: i32 = 0;
    let find_candidates = root.sum_under(&mut sum2, to_delete, &mut candidates);
    find_candidates.2.sort();
    println!("candidates: {:?}",find_candidates.2);
    
}
