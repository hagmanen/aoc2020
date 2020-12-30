/*
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
*/

fn valid_int(value: &str, min: i32, max: i32) -> bool {
    match value.parse::<i32>() {
        Ok(n) => return min <= n && n <= max,
        Err(_) => return false,
    }
}

fn valid_length(value: &str, min_cm: i32, max_cm: i32, min_in: i32, max_in: i32) -> bool {
    let l = value.len();
    let num = &value[..l - 2];
    let unit = &value[l - 2..];
    match unit {
        "cm" => return valid_int(num, min_cm, max_cm),
        "in" => return valid_int(num, min_in, max_in),
        _ => return false,
    }
}

fn valid_hair(value: &str) -> bool {
    let mut chars = value.chars();
    if chars.next().unwrap() != '#' {
        return false;
    }
    for c in chars {
        match c {
            'a'..='f' => continue,
            '0'..='9' => continue,
            _ => return false,
        }
    }
    return true;
}

fn valid_eye(value: &str) -> bool {
    match value {
        "amb" => return true,
        "blu" => return true,
        "brn" => return true,
        "gry" => return true,
        "grn" => return true,
        "hzl" => return true,
        "oth" => return true,
        _ => return false,
    }
}

fn valid_passid(value: &str) -> bool {
    match value.parse::<i64>() {
        Ok(_) => return value.len() == 9,
        Err(_) => return false,
    }
}

fn valid_passport2(passport: String) -> bool {
    let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid) =
        (false, false, false, false, false, false, false);
    for field in passport.split(' ') {
        let mut it = field.split(':');
        let (field_id, field_value) = (it.next().unwrap(), it.next().unwrap());
        match field_id {
            "byr" => byr = valid_int(field_value, 1920, 2002),
            "iyr" => iyr = valid_int(field_value, 2010, 2020),
            "eyr" => eyr = valid_int(field_value, 2020, 2030),
            "hgt" => hgt = valid_length(field_value, 150, 193, 59, 76),
            "hcl" => hcl = valid_hair(field_value),
            "ecl" => ecl = valid_eye(field_value),
            "pid" => pid = valid_passid(field_value),
            "cid" => continue,
            _ => println!("Invalid field: {:?}", field_id),
        }
    }
    return byr && iyr && eyr && hgt && hcl && ecl && pid;
}

fn valid_passport1(passport: String) -> bool {
    let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid) =
        (false, false, false, false, false, false, false);
    for field in passport.split(' ') {
        let field_id = field.split(':').next();
        match field_id.unwrap() {
            "byr" => byr = true,
            "iyr" => iyr = true,
            "eyr" => eyr = true,
            "hgt" => hgt = true,
            "hcl" => hcl = true,
            "ecl" => ecl = true,
            "pid" => pid = true,
            "cid" => continue,
            _ => println!("Invalid field: {:?}", field_id),
        }
    }
    return byr && iyr && eyr && hgt && hcl && ecl && pid;
}

/*
Day 4, part 1: 256
Day 4, part 2: 198
*/

pub fn day4() {
    let contents = std::fs::read_to_string("input4.txt").expect("Failed to read file");
    let mut passport: String = "".to_string();
    let mut valid_passports1 = 0;
    let mut valid_passports2 = 0;
    for line in contents.lines() {
        if line == "" {
            if valid_passport1(passport.clone()) {
                valid_passports1 += 1;
            }
            if valid_passport2(passport) {
                valid_passports2 += 1;
            }
            passport = "".to_string();
        }
        if passport.is_empty() {
            passport = line.to_string();
        } else {
            passport += &(" ".to_owned() + &line.to_string());
        }
    }
    if valid_passport1(passport.clone()) {
        valid_passports1 += 1;
    }
    if valid_passport2(passport) {
        valid_passports2 += 1;
    }

    println!("Day 4, part 1: {}", valid_passports1);
    println!("Day 4, part 2: {}", valid_passports2);
}
