pub fn run(input: String) {
    let first = first(&input);
    println!("First: {first}");
    let second = second(&input);
    println!("second: {second}");
}

fn first(input: &str) -> usize {
    let mut pos = 0;
    for (e, b) in input.as_bytes().windows(4).enumerate() {
        let mut house = vec![0; 4];
        house.clone_from_slice(&b);
        house.sort();
        house.dedup();
        if house.len() == 4 && e >= 1 {
            pos = e + 4;
            break;
        }
    }
    pos
}

fn second(input: &str) -> usize {
    let mut pos = 0;
    for (e, b) in input.as_bytes().windows(14).enumerate() {
        let mut house = vec![0; 14];
        house.clone_from_slice(&b);
        house.sort();
        house.dedup();
        if house.len() == 14 && e >= 1 {
            pos = e + 14;
            break;
        }
    }
    pos
}
