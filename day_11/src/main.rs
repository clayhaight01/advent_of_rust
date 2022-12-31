use std::fs;

#[derive(Clone)]
struct Monkey {
    id: usize, // monkeys id
    items: Vec<i128>, //items that monkey has
    op: (char, i128), //operation +-*/ and value
    test: i128, // divisable by test val
    out: (usize,usize), // (if true, if false)
    ins: i128,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "monkey {}\n",self.id);
        write!(f, "items: {:?}",self.items)
    }
}

fn main() {
    let file_path = "src/day_11_inputs.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines = contents.lines().peekable();
    let mut monkeys: Vec<Monkey> = vec![];
    let mut m_id = 0;
    let mut monkey_business: Vec<i128> = vec![];
    let mut divisor_product = 1;
    while lines.peek().is_some() {
        let line = lines.next();
        let segs = Vec::from_iter(line.unwrap().split(" ").map(String::from));
        if segs[0] == "Monkey" {
            // Instantiation and ID
            let mut monkey = Monkey {
                id: m_id,
                items: vec![],
                op: ('*', 1),
                test: 1,
                out: (0,0),
                ins: 0,
            };
            // Items
            let mut items_str: Vec<String> = Vec::from_iter(lines.next().unwrap().split(" ").map(String::from));
            for _ in 0..4 {items_str.remove(0);}
            for i in items_str {monkey.items.push(i.replace(&[','][..], "").parse::<i128>().unwrap())}
            // Operation
            let m_op: Vec<String> = Vec::from_iter(lines.next().unwrap().split(" ").map(String::from));
            if m_op[7] == "old" {monkey.op = ('^',1);}
            else {monkey.op = (m_op[6].chars().nth(0).unwrap(),m_op[7].parse::<i128>().unwrap());}
            // Test
            let m_test: Vec<String> = Vec::from_iter(lines.next().unwrap().split(" ").map(String::from));
            monkey.test = m_test[5].parse::<i128>().unwrap();
            // True / False
            let m_true: Vec<String> = Vec::from_iter(lines.next().unwrap().split(" ").map(String::from));
            let m_false: Vec<String> = Vec::from_iter(lines.next().unwrap().split(" ").map(String::from));
            monkey.out = (m_true[9].parse::<usize>().unwrap(),m_false[9].parse::<usize>().unwrap());
            monkeys.push(monkey);
            println!("Initialization:");
            println!("{}",monkeys[m_id]);
            m_id += 1;
        }
    }
    for m in &monkeys {
        println!("test: {}",m.test);
        divisor_product *= m.test;
    }
    println!("divisor product: {divisor_product}");
    // Rounds
    for r in 0..10000 {
        println!("*** Round {} ***",r+1);
        let mut m_num = 0;
        for j in 0..monkeys.len() {
            let m = monkeys[j].clone();
            for i in m.items {
                let mut worry: i128 = match m.op.0 {
                    '*' => i * m.op.1,
                    '+' => i + m.op.1,
                    _ => i * i,
                };
                worry %= divisor_product;
                if worry % m.test == 0 {monkeys[m.out.0].items.push(worry);}
                else {monkeys[m.out.1].items.push(worry);}
            }
            monkeys[m_num].ins += monkeys[m_num].items.len() as i128;
            monkeys[m_num].items.clear();
            
            m_num += 1;
        }
        for m in &monkeys {
            println!("{}",m);
            println!("Inspected {} times\n",m.ins);
        }
    }
    for m in &monkeys {monkey_business.push(m.ins);}
    monkey_business.sort_by(|a, b| b.cmp(a));
    println!("The level of monkey business is: {}",monkey_business[0]*monkey_business[1]);
}
