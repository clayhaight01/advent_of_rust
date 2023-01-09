use nom::*;
use nom::character::complete::*;
use atoi::atoi;
use std::cmp::Ordering;

fn main() {
    let div1 = Item::L(vec![Item::L(vec![Item::N(2)])]);
    let div2 = Item::L(vec![Item::L(vec![Item::N(6)])]);
    let mut log: Vec<Item> = include_str!("day_13_inputs.txt")
        .split("\n")
        .filter(|a| *a != "")
        .map(|a| item(a.as_bytes()).unwrap().1)
        .collect();
    log.sort_by(|a, b| a.cmp(b));
    let (mut d1, mut d2) = (0, 0);
    for i in log {
        if div1.cmp(&i) == Ordering::Greater {d1 += 1;}
        if div2.cmp(&i) == Ordering::Greater {d2 += 1;}
    }
    println!("Index Product: {}",(d2+2)*(d1+1));
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum Item {
    N(u8),
    L(Vec<Item>)
}
impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        match (self, other) {
            (Item::N(a), Item::N(b)) => a.cmp(b), // two integers
            (Item::L(a), Item::L(b)) => a.iter().cmp(b), // two lists
            (Item::N(a), Item::L(_)) => Item::L(vec![Item::N(*a)]).cmp(other), // integer and list
            (Item::L(_), Item::N(b)) => self.cmp(&Item::L(vec![Item::N(*b)])), // list and integer
        }
    }
}

named!(pair(&[u8]) -> (Item, Item), separated_pair!(item, tag!("\n"), item));
named!(item(&[u8]) -> Item, alt!(map!(num,Item::N) | map!(list,Item::L)));
named!(num(&[u8]) -> u8, map_opt!(digit0, atoi));
named!(list(&[u8]) -> Vec<Item>, delimited!(char!('['), separated_list0!(char!(','), alt!(map!(num,Item::N) | map!(list,Item::L))), char!(']')));