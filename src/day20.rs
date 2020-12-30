#[derive(Debug, Clone)]
pub struct Tile {
    pub image: Image,
    pub sides: Sides,
}

type Tiles = std::collections::HashMap<u32, Tile>;
type Image = Vec<Vec<bool>>;
type Sides = std::collections::HashSet<u32>;

fn calc_upper_side(image: &[Vec<bool>]) -> u32 {
    let mut side: u32 = 0;
    for i in 0..image.len() {
        side <<= 1;
        if image[0 as usize][i as usize] {
            side |= 1;
        }
    }
    side
}

fn calc_lower_side(image: &[Vec<bool>]) -> u32 {
    let mut side: u32 = 0;
    for i in 0..image.len() {
        side <<= 1;
        if image[image.len() - 1 as usize][i as usize] {
            side |= 1;
        }
    }
    side
}

fn calc_left_side(image: &[Vec<bool>]) -> u32 {
    let mut side: u32 = 0;
    for i in 0..image.len() {
        side <<= 1;
        if image[i as usize][0 as usize] {
            side |= 1;
        }
    }
    side
}

fn calc_right_side(image: &[Vec<bool>]) -> u32 {
    let mut side: u32 = 0;
    for i in 0..image.len() {
        side <<= 1;
        if image[i as usize][image.len() - 1 as usize] {
            side |= 1;
        }
    }
    side
}

fn rotate(image: &[Vec<bool>]) -> Image {
    let mut result = Image::new();
    for y in 0..image.len() {
        let mut row = Vec::new();
        for x in 0..image.len() {
            row.push(image[image.len() - 1 - x][y]);
        }
        result.push(row);
    }
    result
}

fn flip(image: &[Vec<bool>]) -> Image {
    let mut result = Image::new();
    for y in 0..image.len() {
        result.push(image[image.len() - 1 - y].clone());
    }
    result
}

fn calc_sides(image: &[Vec<bool>]) -> Sides {
    let mut sides = Sides::new();
    let mut side1: u32 = 0;
    let mut side2: u32 = 0;
    let mut side3: u32 = 0;
    let mut side4: u32 = 0;
    for i in 0..image.len() {
        side1 <<= 1;
        side2 <<= 1;
        side3 <<= 1;
        side4 <<= 1;
        if image[0 as usize][i as usize] {
            side1 |= 1;
        }
        if image[i as usize][image.len() - 1 as usize] {
            side2 |= 1;
        }
        if image[image.len() - 1 as usize][image.len() - 1 - i as usize] {
            side3 |= 1;
        }
        if image[image.len() - 1 - i as usize][0 as usize] {
            side4 |= 1;
        }
    }
    sides.insert(side1);
    sides.insert(side2);
    sides.insert(side3);
    sides.insert(side4);
    side1 = 0;
    side2 = 0;
    side3 = 0;
    side4 = 0;
    for i in 0..image.len() {
        side1 <<= 1;
        side2 <<= 1;
        side3 <<= 1;
        side4 <<= 1;
        if image[0 as usize][image.len() - 1 - i as usize] {
            side1 |= 1;
        }
        if image[image.len() - 1 - i as usize][image.len() - 1 as usize] {
            side2 |= 1;
        }
        if image[image.len() - 1 as usize][i as usize] {
            side3 |= 1;
        }
        if image[i as usize][0 as usize] {
            side4 |= 1;
        }
    }
    sides.insert(side1);
    sides.insert(side2);
    sides.insert(side3);
    sides.insert(side4);
    sides
}

fn parse_tile_id(input: &str) -> u32 {
    input[5..input.len() - 1].parse().unwrap()
}

fn parse_image(input: std::str::Lines<'_>) -> Image {
    let mut image = Image::new();
    for row in input {
        let mut line = Vec::new();
        for c in row.chars() {
            line.push(c == '#');
        }
        image.push(line);
    }
    image
}

fn parse_tile(input: std::str::Lines<'_>) -> Tile {
    let mut tile = Tile {
        image: parse_image(input),
        sides: Sides::new(),
    };
    tile.sides = calc_sides(&tile.image);
    tile
}

fn parse_tiles(input: std::str::Split<'_, &str>) -> Tiles {
    let mut tiles = Tiles::new();
    for tile in input {
        let mut tile_it = tile.lines();
        tiles.insert(parse_tile_id(tile_it.next().unwrap()), parse_tile(tile_it));
    }
    tiles
}

fn is_corner(sides: &Sides, occurance: &std::collections::HashMap<u32, u32>) -> bool {
    let mut connections: u32 = 0;
    for side in sides {
        if *occurance.get(side).unwrap() > 1 {
            connections += 1;
        }
    }
    connections == 4
}

fn part1(tiles: &Tiles) -> u64 {
    let mut occurance: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    for tile in tiles.values() {
        for side in &tile.sides {
            *occurance.entry(*side).or_insert(0) += 1;
        }
    }
    let mut res: u64 = 1;
    for tile in tiles {
        if is_corner(&tile.1.sides, &occurance) {
            res *= *tile.0 as u64;
        }
    }
    res
}

fn first_corner(tiles: &Tiles, occurance: &std::collections::HashMap<u32, u32>) -> u32 {
    for tile in tiles {
        if is_corner(&tile.1.sides, &occurance) {
            return *tile.0;
        }
    }
    panic!("No corner");
}

fn calc_side_length(tiles: &Tiles) -> u32 {
    let mut res: u32 = 1;
    let nr_of_tiles: u32 = tiles.iter().count() as u32;
    while res * res < nr_of_tiles {
        res += 1;
    }
    res
}

fn match_left(side: u32, image_in: &Image) -> Image {
    let mut image = image_in.clone();
    for _ in 0..4 {
        if calc_left_side(&image) == side {
            return image;
        }
        image = flip(&image);
        if calc_left_side(&image) == side {
            return image;
        }
        image = flip(&image);
        image = rotate(&image);
    }
    return Image::new();
}

fn match_up(side: u32, image_in: &Image) -> Image {
    let mut image = image_in.clone();
    for _ in 0..4 {
        if calc_upper_side(&image) == side {
            return image;
        }
        image = flip(&image);
        if calc_upper_side(&image) == side {
            return image;
        }
        image = flip(&image);
        image = rotate(&image);
    }
    return Image::new();
}

fn next_right_tile(side: u32, tiles: &Tiles) -> (u32, Image) {
    for tile in tiles {
        let image = match_left(side, &tile.1.image);
        if image.len() > 0 {
            return (*tile.0, image);
        }
    }
    panic!("No match");
}

fn next_lower_tile(side: u32, tiles: &Tiles) -> (u32, Image) {
    for tile in tiles {
        let image = match_up(side, &tile.1.image);
        if image.len() > 0 {
            return (*tile.0, image);
        }
    }
    panic!("No match");
}

fn match_creature(y: usize, x: usize, image: &mut Image) -> bool {
    if !image[y][x + 18 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x] {
        return false;
    }
    if !image[y + 1 as usize][x + 5 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 6 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 11 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 12 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 17 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 18 as usize] {
        return false;
    }
    if !image[y + 1 as usize][x + 19 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 1 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 4 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 7 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 10 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 13 as usize] {
        return false;
    }
    if !image[y + 2 as usize][x + 16 as usize] {
        return false;
    }

    image[y][x + 18 as usize] = false;
    image[y + 1 as usize][x] = false;
    image[y + 1 as usize][x + 5 as usize] = false;
    image[y + 1 as usize][x + 6 as usize] = false;
    image[y + 1 as usize][x + 11 as usize] = false;
    image[y + 1 as usize][x + 12 as usize] = false;
    image[y + 1 as usize][x + 17 as usize] = false;
    image[y + 1 as usize][x + 18 as usize] = false;
    image[y + 1 as usize][x + 19 as usize] = false;
    image[y + 2 as usize][x + 1 as usize] = false;
    image[y + 2 as usize][x + 4 as usize] = false;
    image[y + 2 as usize][x + 7 as usize] = false;
    image[y + 2 as usize][x + 10 as usize] = false;
    image[y + 2 as usize][x + 13 as usize] = false;
    image[y + 2 as usize][x + 16 as usize] = false;
    true
}
/*
                  #
012345678901234567890
#    ##    ##    ###
012345678901234567890
 #  #  #  #  #  #
*/

fn find_creature(image_in: &Image) -> (u32, Image) {
    let mut image = image_in.clone();
    let mut m: u32 = 0;
    for y in 0..image.len() - 3 {
        for x in 0..image.len() - 20 {
            if match_creature(y, x, &mut image) {
                m += 1;
            }
        }
    }
    (m, image)
}

fn create_mega_image(full: &Vec<Vec<Image>>) -> Image {
    let mut mega_image = Image::new();
    let int_row_size: u32 = full[0][0][0].len() as u32 - 2;
    let row_size: u32 = full[0].len() as u32 * int_row_size;
    for y in 0..row_size {
        let mut mega_row = Vec::new();
        for x in 0..row_size {
            let y_i = y / int_row_size;
            let y_p = y % int_row_size;
            let x_i = x / int_row_size;
            let x_p = x % int_row_size;
            mega_row.push(
                full[y_i as usize][x_i as usize][y_p as usize + 1 as usize]
                    [x_p as usize + 1 as usize],
            );
        }
        mega_image.push(mega_row);
    }
    mega_image
}

fn part2(tiles_in: &Tiles) -> u64 {
    let mut tiles = tiles_in.clone();
    let side_len = calc_side_length(tiles_in);
    let mut occurance: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    for tile in tiles.values() {
        for side in &tile.sides {
            *occurance.entry(*side).or_insert(0) += 1;
        }
    }
    let first_corner = first_corner(&tiles, &occurance);
    let mut first_image = tiles.get(&first_corner).unwrap().image.clone();
    while !(*occurance.get(&calc_upper_side(&first_image)).unwrap() == 1
        && *occurance.get(&calc_left_side(&first_image)).unwrap() == 1)
    {
        first_image = rotate(&first_image);
    }
    let mut full_image = Vec::new();
    let mut first_row = Vec::new();
    first_row.push(first_image);
    tiles.remove(&first_corner);
    for _ in 0..side_len - 1 {
        let side = calc_right_side(first_row.last().unwrap());
        let next_tile = next_right_tile(side, &tiles);
        tiles.remove(&next_tile.0);
        first_row.push(next_tile.1);
    }
    full_image.push(first_row);
    for _ in 0..side_len - 1 {
        let mut next_row = Vec::new();
        for above_image in full_image.last().unwrap() {
            let side = calc_lower_side(above_image);
            let next_tile = next_lower_tile(side, &tiles);
            tiles.remove(&next_tile.0);
            next_row.push(next_tile.1);
        }
        full_image.push(next_row);
    }

    let mut mega_image = create_mega_image(&full_image);
    for _ in 0..4 {
        let t1 = find_creature(&mega_image);
        if t1.0 > 0 {
            mega_image = t1.1;
            break;
        }
        mega_image = flip(&mega_image);
        let t2 = find_creature(&mega_image);
        if t2.0 > 0 {
            mega_image = t2.1;
            break;
        }
        mega_image = flip(&mega_image);
        mega_image = rotate(&mega_image);
    }

    let mut count: u64 = 0;
    for row in mega_image {
        for pix in row {
            if pix {
                count += 1;
            }
        }
    }
    count
}

/*
Day 20, part 1: 108603771107737
Day 20, part 2: 2129
*/

pub fn day20() {
    let contents = std::fs::read_to_string("input20.txt").expect("Failed to read file");
    let tiles: Tiles = parse_tiles(contents.split("\n\n"));
    println!("Day 20, part 1: {}", part1(&tiles));
    println!("Day 20, part 2: {}", part2(&tiles));
}
