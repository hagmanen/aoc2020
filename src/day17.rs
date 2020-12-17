#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

type Space = std::collections::HashMap<Point, bool>;

fn init_space(lines: std::str::Lines) -> Space {
    let mut space: Space = Space::new();
    for (y, line) in lines.enumerate() {
        for (x, _) in line.chars().enumerate().filter(|x| x.1 == '#') {
            space.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                },
                true,
            );
        }
    }
    space
}

fn count_neighbors(p: &Point, space: &Space, wd: i32) -> u32 {
    let mut res: u32 = 0;
    for x in (p.x - 1)..(p.x + 2) {
        for y in (p.y - 1)..(p.y + 2) {
            for z in (p.z - 1)..(p.z + 2) {
                for w in (p.w - wd)..(p.w + wd + 1) {
                    if *space.get(&Point { x, y, z, w }).unwrap_or(&false) {
                        res += 1;
                    }
                }
            }
        }
    }
    if *space.get(p).unwrap() {
        res -= 1;
    }
    res
}

fn mutate(space: &Space, wd: i32) -> Space {
    let mut new_space: Space = Space::new();
    for p in space {
        let neighbors: u32 = count_neighbors(&p.0, space, wd);
        if neighbors == 3 || (*p.1 && neighbors == 2) {
            new_space.insert(*p.0, true);
        }
    }
    new_space
}

fn init_neighbors(p: &Point, space: &mut Space, wd: i32) {
    for x in (p.x - 1)..(p.x + 2) {
        for y in (p.y - 1)..(p.y + 2) {
            for z in (p.z - 1)..(p.z + 2) {
                for w in (p.w - wd)..(p.w + wd + 1) {
                    space.entry(Point { x, y, z, w }).or_insert(false);
                }
            }
        }
    }
}

fn expand(space: &Space, wd: i32) -> Space {
    let mut new_space: Space = Space::new();
    for p in space {
        if *p.1 {
            new_space.insert(*p.0, true);
            init_neighbors(p.0, &mut new_space, wd);
        }
    }
    new_space
}

fn solve(init_space: &Space, wd: i32) -> u32 {
    let mut space: Space = init_space.clone();
    for _ in 0..6 {
        space = expand(&space, wd);
        space = mutate(&space, wd);
    }
    let mut c: u32 = 0;
    for p in space {
        if p.1 {
            c += 1;
        }
    }
    c
}

/*
Day 17, part 1: 348
Day 17, part 2: 2236
*/

pub fn day17() {
    let contents = std::fs::read_to_string("input17.txt").expect("Failed to read file");

    let space: Space = init_space(contents.lines());
    println!("Day 17, part 1: {}", solve(&space, 0));
    println!("Day 17, part 2: {}", solve(&space, 1));
}
