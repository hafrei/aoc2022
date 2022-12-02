pub fn run(input: String) {
    let first = part_one(input.clone());
    println!("{}", first);
    let second = part_two(input);
    println!("{}", second);
}

fn part_one(input: String) -> u32 {
    let mut max: u32 = 0;
    let mut cache: u32 = 0;

    for i in input.lines() {
        //TODO: if the end of the file isn't a /n, don't drop the last part of the input
        if i.len() == 0 {
            if max < cache {
                max = cache;
            }
            cache = 0;
        } else {
            cache += i.parse::<u32>().unwrap();
        }
    }
    max
}

fn part_two(input: String) -> u32 {
    let mut sums = Vec::new();
    let mut cache = 0;

    for i in input.lines() {
        //TODO: if the end of the file isn't a /n, don't drop the last part of the input
        if i.len() == 0 {
            sums.push(cache);
            cache = 0;
        } else {
            cache += i.parse::<u32>().unwrap();
        }
    }

    sums.sort();
    sums.reverse();
    sums.truncate(3);
    sums.into_iter().sum()
}
