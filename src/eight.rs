use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y) & (self.height == other.height)
    }
}

#[derive(Debug, Clone, Copy)]
struct Copter {
    direction: Flightpath,
    step_by: usize,
    rotate: usize,
    range_lower: u32,
    range_higher: u32,
    len: usize,
}

//Start at opposite, travel in this direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Flightpath {
    Down,
    Up,
    Left,
    Right,
}

pub fn run(input: String) {
    // println!("{input}");
    let (forest, x, y) = get_forest(input);
    let mut walking: VecDeque<u32> = forest.clone().into();
    let first_copter = Copter {
        direction: Flightpath::Down,
        step_by: x,
        rotate: 1,
        range_lower: 0,
        range_higher: y,
        len: x,
    };
    let second_copter = Copter {
        direction: Flightpath::Right,
        step_by: 1,
        rotate: x,
        range_lower: 0,
        range_higher: y,
        len: x,
    };
    let third_copter = Copter {
        direction: Flightpath::Left,
        step_by: 1,
        rotate: x,
        range_lower: 0,
        range_higher: y,
        len: x,
    };
    let fourth_copter = Copter {
        direction: Flightpath::Up,
        step_by: x,
        rotate: 1,
        range_lower: 0,
        range_higher: y,
        len: x,
    };
    let mut visible = Vec::new();
    visible.append(&mut find_visible(&mut walking, first_copter));
    // let mut walking: VecDeque<u32> = forest.clone().into(); //Ooof, this sucks
    // visible.append(&mut find_visible(&mut walking, second_copter));
    // let mut walking: VecDeque<u32> = forest.clone().into(); //TODO: rotate back properly
    // visible.append(&mut find_visible(&mut walking, third_copter));
    // let mut walking: VecDeque<u32> = forest.clone().into();
    // visible.append(&mut find_visible(&mut walking, fourth_copter));
    visible.sort();
    visible.dedup();
    println!("{}", visible.len()); //619 is too low, 1907 is too high
    // for t in visible.iter() { //wouldn't it be useful if there could be a coloured visualization
    // for this?
    //     println!("{t:?}");
    // }
}

fn find_visible(walking: &mut VecDeque<u32>, copter: Copter) -> Vec<Tree> {
    let mut total_visible = Vec::new();
    for ypos in copter.range_lower..copter.range_higher {
        let mut last_visible = 0;

        if copter.direction == Flightpath::Right || copter.direction == Flightpath::Down {
        let mut x = 0;
            for n in walking
                .make_contiguous()
                .iter()
                .step_by(copter.step_by)
                .take(copter.len)
            {
                compare_visible(*n, &mut last_visible, &copter, &mut total_visible, x, ypos, 0u32);
                    if last_visible == 9 {
                        break
                    }
                x += 1;
            }
        } else {
            let mut x = copter.len.try_into().unwrap();
            for n in walking
                .make_contiguous()
                .iter()
                .step_by(copter.step_by)
                .take(copter.len)
                .rev()
            {
                compare_visible(*n, &mut last_visible, &copter, &mut total_visible, x, ypos, 5u32);
                    if last_visible == 9 {
                        break
                    }
                x -= 1;
            }
        }
        walking.rotate_left(copter.rotate);
    }
    total_visible
}

fn compare_visible(
    n: u32,
    last_visible: &mut u32,
    copter: &Copter,
    total_visible: &mut Vec<Tree>,
    x: u32,
    y: u32,
    max_band: u32
) {
    // println!("{n} vs {}", *last_visible);
    if n > *last_visible || (x == max_band) {
        match copter.direction {
            Flightpath::Up => {
                let tree = Tree {
                    y: x-1,
                    x: y,
                    height: n,
                };
                // println!("{tree:?}");
                total_visible.push(tree);
            }
            Flightpath::Right => {
                let tree = Tree {
                    height: n,
                    y,
                    x,
                };
                // println!("{tree:?}");
                total_visible.push(tree);
            }
            Flightpath::Left => {
                let tree = Tree {
                    height: n,
                    x: x-1,
                    y,
                };
                // println!("{tree:?}");
                total_visible.push(tree);
            }
            Flightpath::Down => {
                let tree = Tree {
                    height: n,
                    y: x,
                    x: y,
                };
                println!("{tree:?}");
                total_visible.push(tree);
            }
        }
        *last_visible = n;
    }
}

fn get_forest(input: String) -> (Vec<u32>, usize, u32) {
    let mut x = 0;
    let forest: Vec<u32> = input
        .split('\n')
        .map(|l| {
            if x == 0 {
                x = l.len();
            }
            l.chars()
                .map(|x| x.to_digit(10))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect();

    let y = forest.len() / x;

    (forest, x.try_into().unwrap(), y.try_into().unwrap())
}
