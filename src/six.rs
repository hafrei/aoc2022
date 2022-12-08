pub fn run(input: String) {
    let first = first(&input);
    println!("First: {first}");
}

fn first(input: &str) -> usize  {
    let mut pos = 0;
    for (e, b) in input.as_bytes().windows(4).enumerate() {
        let mut reference = vec![0;4];
        let mut house = vec![0;4];
        house.clone_from_slice(&b);
        reference.clone_from_slice(&b);
        house.sort();
        house.dedup();
        if house.len() == 4 && e >= 1 {
            pos = e+4;
            break
        }
    }
    pos
}
