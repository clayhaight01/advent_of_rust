use std::cmp::Ordering;
fn main() {
    let a = vec![1,2,3];
    let b = vec![0,3,4,5];
    let res: bool = a.cmp(&b) == Ordering::Less;
    println!("Test: {res}");
}
