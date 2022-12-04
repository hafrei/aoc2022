use rayon::prelude::*;

pub fn run(input: String) {
    let first = first(input.clone());
    println!("First: {first}");
    let second = second(input);
    println!("Second: {second}");
}

fn first(input: String) -> u32 {
    input
        .par_lines()
        .map(|i| {
            let (lhs, rhs) = i.split_at(i.len() / 2);
            let mut matched: Vec<char> = lhs.par_chars().filter(|li| rhs.contains(*li)).collect();
            matched.sort();
            matched.dedup();
            matched
                .into_iter()
                .map(|i| determine_value(i))
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
}

fn second(input: String) -> u32 {
    let inp = input
        .par_lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    inp.par_chunks(3)
        .map(|chunk| {
            let mut matched: Vec<char> = chunk[0]
                .par_chars()
                .filter(|li| chunk[1].contains(*li) && chunk[2].contains(*li))
                .collect();
            matched.sort();
            matched.dedup();
            matched
                .into_iter()
                .map(|i| determine_value(i))
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
}

fn determine_value(input: char) -> u32 {
    let ret = if input.is_lowercase() {
        u32::from(input) - 96
    } else {
        u32::from(input) - 38
    };
    // println!("{input}: {ret}");
    ret
}
