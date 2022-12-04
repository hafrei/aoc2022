use rayon::prelude::*;

pub fn run(input: String) {
    let first = first(input.clone());
    println!("First: {first}");
    let second = second(input);
    println!("Second: {second}");
}

fn first(input: String) -> usize {
    input
        .par_lines()
        .map(|l| parse(l.to_string()))
        .filter(|[a, b]| (a[0] <= b[0] && a[1] >= b[1]) || (b[0] <= a[0] && b[1] >= a[1]))
        .count()
}

fn second(input: String) -> usize {
    input
        .par_lines()
        .map(|l| parse(l.to_string()))
        .filter(|[a, b]| (a[0] <= b[1] && a[0] >= b[1]) || (b[0] <= a[1] && b[1] >= a[0]))
        .count()
}

fn parse(input: String) -> [[u32; 2]; 2] {
    let ret: Vec<u32> = input
        .replace("-", " ")
        .replace(",", " ")
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    [[ret[0], ret[1]], [ret[2], ret[3]]]
}
