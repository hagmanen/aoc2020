fn parse_grid(lines: std::str::Lines) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }
    return result;
}

fn occupied_in_dir(grid: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32, part: u32) -> u32 {
    if x < 0
        || x >= grid[0].len() as i32
        || y < 0
        || y >= grid.len() as i32
        || grid[y as usize][x as usize] == 'L'
    {
        return 0;
    }
    if grid[y as usize][x as usize] == '#' {
        return 1;
    }
    if part == 1 {
        return 0;
    }
    return occupied_in_dir(grid, x + dx, y + dy, dx, dy, part);
}

fn occupied_neibours(grid: &Vec<Vec<char>>, x: i32, y: i32, part: u32) -> u32 {
    return occupied_in_dir(grid, x, y - 1, 0, -1, part)
        + occupied_in_dir(grid, x, y + 1, 0, 1, part)
        + occupied_in_dir(grid, x - 1, y - 1, -1, -1, part)
        + occupied_in_dir(grid, x - 1, y, -1, 0, part)
        + occupied_in_dir(grid, x - 1, y + 1, -1, 1, part)
        + occupied_in_dir(grid, x + 1, y - 1, 1, -1, part)
        + occupied_in_dir(grid, x + 1, y, 1, 0, part)
        + occupied_in_dir(grid, x + 1, y + 1, 1, 1, part);
}

fn permute_grid(grid: Vec<Vec<char>>, part: u32) -> (bool, Vec<Vec<char>>) {
    let mut result = grid.clone();
    let mut mutated = false;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'L' && occupied_neibours(&grid, x as i32, y as i32, part) == 0 {
                result[y][x] = '#';
                mutated = true;
            }
            if grid[y][x] == '#' && occupied_neibours(&grid, x as i32, y as i32, part) >= (3 + part)
            {
                result[y][x] = 'L';
                mutated = true;
            }
        }
    }
    return (mutated, result);
}

fn part(text: &String, part: u32) {
    let mut t = (true, parse_grid(text.lines()));
    while t.0 {
        t = permute_grid(t.1, part);
    }
    let mut oc = 0;
    for l in &t.1 {
        for c in l {
            if *c == '#' {
                oc += 1;
            }
        }
    }
    println!("Day 11, part {}: {}", part, oc);
}

/*
Day 11, part 1: 2441
Day 11, part 2: 2190
*/

pub fn day11() {
    let contents = std::fs::read_to_string("input11.txt").expect("Failed to read file");
    part(&contents, 1);
    part(&contents, 2);
}
