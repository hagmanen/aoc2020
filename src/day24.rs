#[derive(Debug, Clone)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    pub fn iterator() -> std::slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NE,
            Direction::NW,
        ];
        DIRECTIONS.iter()
    }
}

type Directions = Vec<Direction>;
type Grid = std::collections::HashMap<Point, bool>;
type Points = std::collections::HashSet<Point>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::convert::From<&Direction> for Point {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::E => Point { x: 1, y: 0 },
            Direction::SE => Point { x: 1, y: -1 },
            Direction::SW => Point { x: 0, y: -1 },
            Direction::W => Point { x: -1, y: 0 },
            Direction::NW => Point { x: -1, y: 1 },
            Direction::NE => Point { x: 0, y: 1 },
        }
    }
}

fn parse_directions(input: &str) -> Directions {
    let mut res = Vec::new();
    let mut chars = input.chars();
    loop {
        let c = chars.next();
        if c == None {
            return res;
        }
        let c1 = c.unwrap();
        match c1 {
            'e' => res.push(Direction::E),
            's' => res.push(if chars.next().unwrap() == 'e' {
                Direction::SE
            } else {
                Direction::SW
            }),
            'w' => res.push(Direction::W),
            'n' => res.push(if chars.next().unwrap() == 'e' {
                Direction::NE
            } else {
                Direction::NW
            }),
            _ => panic!("Unknown char {}", c1),
        }
    }
}

fn calc_point(directions: Directions) -> Point {
    let mut point = Point { x: 0, y: 0 };
    for direction in &directions {
        point += direction.into();
    }
    point
}

fn part1(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.lines() {
        let directions = parse_directions(line);
        let point = calc_point(directions);
        *grid.entry(point).or_insert(false) ^= true;
    }
    grid
}

fn touch_neighbours(mut grid: Grid) -> Grid {
    let mut new_points = Points::new();
    for point in grid.iter().filter(|s| *s.1).map(|x| x.0) {
        for direction in Direction::iterator() {
            new_points.insert(*point + direction.into());
        }
    }
    for point in new_points {
        grid.entry(point).or_insert(false);
    }
    grid
}

fn count_neighbors(grid: &Grid, point: &Point) -> usize {
    Direction::iterator()
        .filter(|&d| *grid.get(&(*point + d.into())).unwrap_or(&false))
        .count()
}

fn mutate_grid(grid: Grid) -> Grid {
    let mut new_grid = Grid::new();
    for point in &grid {
        let neighbors = count_neighbors(&grid, &point.0);
        if (*point.1 && neighbors == 1) || neighbors == 2 {
            new_grid.insert(*point.0, true);
        }
    }
    new_grid
}

fn part2(mut grid: Grid) -> usize {
    for _ in 0..100 {
        grid = touch_neighbours(grid);
        grid = mutate_grid(grid);
    }
    grid.iter().filter(|s| *s.1).count()
}

/*
Day 24, part 1: 488
Day 24, part 2: 4118
*/

pub fn day24() {
    let contents = std::fs::read_to_string("input24.txt").expect("Failed to read file");
    let day1_grid = part1(&contents);
    println!(
        "Day 24, part 1: {}",
        day1_grid.iter().filter(|s| *s.1).count()
    );
    println!("Day 24, part 2: {}", part2(day1_grid));
}
