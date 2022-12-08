pub fn run(input: String) {
    let first = first(input.clone());
    println!("First: {first}");
}

#[derive(Debug)]
struct CraneYard {
    stacks: Vec<Stack>,
    orders: Vec<Orders>,
}

#[derive(Debug)]
struct Orders {
    origin: usize,
    dest: usize,
    times: usize,
}

impl CraneYard {
    fn new() -> Self {
        Self {
            stacks: Vec::new(),
            orders: Vec::new(),
        }
    }
    fn populate(&mut self, input: &str) {
        let stack_count = CraneYard::extract_stack_count(input);
        let mut orders: Vec<Orders> = Vec::new();
        Self::extract_orders(input.clone().to_string(), &mut orders);
        let piles: Vec<&str> = input
            .lines()
            .filter(|x| x.contains(char::is_alphabetic) && !x.contains(char::is_numeric))
            .rev()
            .collect();

        let mut stacks = Vec::new();
        let mut getter: usize = 1;

        for _ in 0..stack_count {
            let mut stack = Stack::new();
            for x in 0..piles.len() {
                let hup = piles[x as usize].chars().nth(getter as usize).unwrap();
                if hup.is_alphabetic() {
                    stack.crates.push(hup);
                }
            }
            stacks.push(stack);
            getter += 4;
        }
        self.stacks.append(&mut stacks);
        self.orders.append(&mut orders);
    }
    fn extract_stack_count(input: &str) -> u32 {
        input
            .lines()
            .filter(|x| x.contains(char::is_numeric))
            .take(1)
            .map(|x| x.split_whitespace())
            .flatten()
            .map(|x| x.parse::<u32>())
            .map(Result::unwrap)
            .collect::<Vec<u32>>()
            .pop()
            .expect("How is this not a number")
    }

    fn extract_orders(input: String, orders: &mut Vec<Orders>) {
        orders.append(
            &mut input
                .lines()
                .filter(|x| x.contains(char::is_numeric) && x.contains(char::is_alphabetic))
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<usize>())
                        .filter(Result::is_ok)
                        .map(Result::unwrap)
                        .collect::<Vec<usize>>()
                })
                .map(|x| Orders {
                    origin: x[1],
                    dest: x[2],
                    times: x[0],
                })
                .collect::<Vec<Orders>>(),
        )
    }

    fn run_orders(&mut self) {
        for order in self.orders.iter() {
            for _ in 0..order.times {
                let transit = self.stacks[order.origin - 1]
                    .crates
                    .pop()
                    .expect("What wait how");
                self.stacks[order.dest - 1].crates.push(transit);
            }
        }
    }

    fn top_crates(&mut self) -> String {
        let mut res = Vec::new();
        for stack in self.stacks.iter_mut() {
            res.push(stack.crates.pop().unwrap());
        }
        String::from_iter(res.iter())
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
}

fn first(input: String) -> String {
    let mut yard = CraneYard::new();
    yard.populate(&input);
    yard.run_orders();
    yard.top_crates()
}
