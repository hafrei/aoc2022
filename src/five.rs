pub fn run(input: String) {
    let first = first(input.clone());
    println!("First: {first}");
}

#[derive(Debug)]
struct CraneYard(pub Vec<Stack>);

impl CraneYard {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn populate(&mut self, input: &str) {
        let stack_count = CraneYard::extract_stack_count(input);
        let piles: Vec<&str> = input
            .lines()
            .filter(|x| x.contains(char::is_alphabetic) && !x.contains(char::is_numeric))
            .inspect(|x| println!("{x}"))
            .rev()
            .collect();

        let mut stacks = Vec::new();

        //Stack_cound is vertical
        //piles is horazontal
        let mut getter: usize = 1;

        for y in 0..stack_count {
            let mut stack = Stack::new();
            for x in 0..piles.len() {
                let hup = piles[x as usize].chars().nth(getter as usize).unwrap();
                if hup.is_alphabetic() {
                stack.crates.push(
                    hup
                );
                }
            }
            stacks.push(stack);
            getter +=4;
        }
        self.0.append(&mut stacks);
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

fn first(input: String) -> u32 {
    let mut yard = CraneYard::new();
    yard.populate(&input);
    println!("{:?}", yard.0);
    0
}
