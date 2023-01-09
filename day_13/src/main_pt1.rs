use nom::*;
use nom::character::complete::*;
use atoi::atoi;
use std::cmp::Ordering;

fn main() {
    println!("{}",
        include_str!("day_13_inputs.txt")
        .split("\n\n")
        .map(|a| pair(a.as_bytes()).unwrap().1)
        .enumerate()
        .filter(|(_,(a,b))| a.cmp(b) == Ordering::Less)
        .map(|(i, _)| i+1)
        .sum::<usize>()
    );
}

#[derive(Eq, PartialEq, PartialOrd)]
enum Item {
    I(u8),
    L(Vec<Item>)
}
impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        match (self, other) {
            (Item::I(a), Item::I(b)) => a.cmp(b), // two integers
            (Item::L(a), Item::L(b)) => a.iter().cmp(b.iter()), //two lists
            (Item::I(a), Item::L(_)) => Item::L(vec![Item::I(*a)]).cmp(other), // integer and list
            (Item::L(_), Item::I(b)) => self.cmp(&Item::L(vec![Item::I(*b)])), // list and integer
        }
    }
}

named!(pair(&[u8]) -> (Item, Item), separated_pair!(alt!(map!(num,Item::I) | map!(list,Item::L)),tag!("\n"),alt!(map!(num,Item::I) | map!(list,Item::L))));
named!(num(&[u8]) -> u8, map_opt!(digit0, atoi));
named!(list(&[u8]) -> Vec<Item>, delimited!(char!('['), separated_list0!(char!(','), alt!(map!(num,Item::I) | map!(list,Item::L))), char!(']')));