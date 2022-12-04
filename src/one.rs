use rayon::prelude::*;

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
        if i.is_empty() {
            if max < cache {
                max = cache;
            }
            cache = 0;
        } else {
            cache += i.parse::<u32>().unwrap();
        }
    }

    if max < cache {
        max = cache;
    }

    max
}

fn part_two(input: String) -> u32 {
    let mut sums = Vec::new();
    let mut cache = 0;

    for i in input.lines() {
        if i.is_empty() {
            sums.push(cache);
            cache = 0;
        } else {
            cache += i.parse::<u32>().unwrap();
        }
    }
    if cache != 0 {
        sums.push(cache);
    }

    sums.par_sort_by(|a, b| b.cmp(a));
    sums.truncate(3);
    sums.into_iter().sum()
}
