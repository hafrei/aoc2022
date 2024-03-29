use ansiterm::Colour;
use core::cmp::Ordering;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
struct Tree {
    x: u32,
    y: u32,
    height: u32,
    colour: Colour,
    scenic_score: Option<u32>,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        // (self.x == other.x) & (self.y == other.y) & (self.height == other.height)
        self.x == other.x
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, &self.y, self.height).cmp(&(other.x, &other.y, other.height))
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colour.paint(self.height.to_string().as_str()))
    }
}

impl Eq for Tree {}

struct Forest<'a, T: 'a> {
    x_max: u32,
    trees: &'a [T],
}

impl<'a, T: fmt::Display + 'a> fmt::Display for Forest<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut len_counter = 0;
        for item in self.trees {
            write!(f, "{}", item)?;
            if len_counter == self.x_max {
                write!(f, "\n")?;
                len_counter = 0;
            } else {
                len_counter += 1;
            }
        }
        Ok(())
    }
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
    let mut trees = Vec::new();
    get_colour_forest(input, &mut trees);
    let (max_x, max_y) = {
        let Tree { x, y, .. } = trees.iter().last().unwrap();
        (*x, *y)
    };
    print_forest(&mut trees);
    find_visible(&mut trees, Flightpath::Down, 0, 0, max_x, max_y);
    find_visible(&mut trees, Flightpath::Up, 0, 0, max_x, max_y);
    find_visible(&mut trees, Flightpath::Left, 0, 0, max_x, max_y);
    find_visible(&mut trees, Flightpath::Right, 0, 0, max_x, max_y);
    print_forest(&mut trees);
    let visible = trees
        .iter()
        .filter(|x| x.colour == Colour::BrightGreen)
        .count();
    println!("There are {visible} trees");
    let mut trees = trees
        .into_iter()
        .map(|mut x| {
            x.colour = Colour::DarkGray;
            x
        })
        .collect::<Vec<Tree>>();

    println!();
    print_forest(&mut trees);
    todo!();
}

fn print_forest(trees: &mut Vec<Tree>) {
    let Tree { x, .. } = trees.iter().last().unwrap();
    let forest = Forest {
        x_max: *x,
        trees: &trees,
    };
    println!("{}", forest);
}

fn find_visible(
    trees: &mut Vec<Tree>,
    path: Flightpath,
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
) {
    if path == Flightpath::Right {
        for i in min_x..=max_x {
            let mut last_visible = 0;
            for n in trees.iter_mut().filter(|x| x.y == i) {
                // println!("{n:?}\t{last_visible}");
                compare_visible(n, &mut last_visible, n.x, 0);
                if last_visible == 9 {
                    break;
                }
            }
        }
    } else if path == Flightpath::Down {
        for i in min_y..=max_y {
            let mut last_visible = 0;
            for n in trees.iter_mut().filter(|x| x.x == i) {
                // println!("{n:?}\t{last_visible}");
                compare_visible(n, &mut last_visible, n.y, 0);
                if last_visible == 9 {
                    break;
                }
            }
        }
    } else if path == Flightpath::Up {
        for i in min_y..=max_y {
            let mut last_visible = 0;
            for n in trees.iter_mut().filter(|x| x.x == i).rev() {
                // println!("{n:?}\t{last_visible}");
                compare_visible(n, &mut last_visible, n.y, max_y);
                if last_visible == 9 {
                    break;
                }
            }
        }
    } else {
        for i in min_x..=max_x {
            let mut last_visible = 0;
            for n in trees.iter_mut().filter(|x| x.y == i).rev() {
                // println!("{n:?}\t{last_visible}");
                compare_visible(n, &mut last_visible, n.x, max_x);
                if last_visible == 9 {
                    break;
                }
            }
        }
    }
}

fn compare_visible(n: &mut Tree, last_visible: &mut u32, facing_pos: u32, max_pos: u32) {
    if n.height > *last_visible || facing_pos == max_pos {
        n.colour = Colour::BrightGreen;
        *last_visible = n.height;
    }
}

fn get_colour_forest(input: String, forest: &mut Vec<Tree>) {
    for (i, y) in input.split('\n').enumerate() {
        for (j, x) in y.chars().enumerate() {
            let tree = Tree {
                x: j as u32,
                y: i as u32,
                height: (x.to_digit(10).unwrap() as u32),
                colour: Colour::DarkGray,
                scenic_score: None,
            };
            forest.push(tree);
        }
    }
}
