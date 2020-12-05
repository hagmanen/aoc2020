fn count_trees(lines : &Vec<String>, xinc : usize, yinc: usize) -> u64 {
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut trees : u64 = 0;
    for line in lines {
        if y > 0 {
            y += 1;
            y %= yinc;
            continue;
        }
        let rep : usize = line.len();
        if line.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x += xinc;
        x %= rep;
        y += 1;
        y %= yinc;
    }
    return trees;
}

fn lines_to_vector(lines : std::str::Lines) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    for line in lines {
        result.push(line.to_string());
    }
    return result;
}

/*
Day 3, part 1: 169
Day 3, part 2: 7560370818
*/

pub fn day3() {
    let contents = std::fs::read_to_string("input3.txt")
        .expect("Failed to read file");
    let lines = lines_to_vector(contents.lines());

    println!("Day 3, part 1: {}", count_trees(&lines, 3, 1));
    println!("Day 3, part 2: {}", count_trees(&lines, 1, 1) *
                                  count_trees(&lines, 3, 1) *
                                  count_trees(&lines, 5, 1) *
                                  count_trees(&lines, 7, 1) *
                                  count_trees(&lines, 1, 2));
}
