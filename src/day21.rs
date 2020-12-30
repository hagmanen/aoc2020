#[derive(Debug)]
struct Food {
    foreign: Ingredients,
    translated: Ingredients,
}

type FoodList = Vec<Food>;
type Ingredients = std::collections::HashSet<String>;
type PossibleTranslations = std::collections::HashMap<String, Ingredients>;
type PossibleTranslationsIter<'a> = std::collections::hash_map::Iter<'a, String, Ingredients>;
type Translations = std::collections::HashMap<String, String>;

fn parse_ingredients(input: &str, delimiter: &str) -> Ingredients {
    input.split(delimiter).map(|i| i.to_string()).collect()
}

fn parse_food(mut input: String) -> Food {
    input.truncate(input.len() - 1);
    let mut it = input.split(" (contains ");
    Food {
        foreign: parse_ingredients(it.next().unwrap(), " "),
        translated: parse_ingredients(it.next().unwrap(), ", "),
    }
}

fn parse_foods(input: String) -> FoodList {
    let mut list = FoodList::new();
    for line in input.lines() {
        list.push(parse_food(line.to_string()));
    }
    list
}

fn get_possible_translations(foods: &[Food]) -> PossibleTranslations {
    let mut result = PossibleTranslations::new();
    for food in foods {
        for translated_ingredient in &food.translated {
            let intersection = result
                .entry(translated_ingredient.clone())
                .or_insert_with(|| food.foreign.clone())
                .intersection(&food.foreign)
                .cloned()
                .collect();
            result.insert(translated_ingredient.clone(), intersection);
        }
    }
    result
}

fn get_translations(
    mut possible_translations: PossibleTranslationsIter,
    picked_ingredients: Ingredients,
) -> Option<Translations> {
    match possible_translations.next() {
        None => Some(Translations::new()),
        Some((translated, possible_foreign_ingredients)) => {
            for possible_foreign_ingredient in possible_foreign_ingredients {
                if picked_ingredients.contains(possible_foreign_ingredient) {
                    continue;
                }
                let mut new_picked = picked_ingredients.clone();
                new_picked.insert(possible_foreign_ingredient.clone());
                match get_translations(possible_translations.clone(), new_picked) {
                    None => continue,
                    Some(mut translations) => {
                        translations
                            .insert(translated.clone(), possible_foreign_ingredient.clone());
                        return Some(translations);
                    }
                }
            }
            None
        }
    }
}

fn part1(foods: &[Food], allergens: Ingredients) -> u32 {
    let mut res: u32 = 0;
    for ingredients in foods {
        for ingredient in &ingredients.foreign {
            if !allergens.contains(ingredient) {
                res += 1;
            }
        }
    }
    res
}

fn part2(translations: Translations) -> String {
    let mut t: Vec<(String, String)> = translations.into_iter().collect();
    t.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    t.iter()
        .map(|x| x.1.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/*
Day 21, part 1: 2614
Day 21, part 2: qhvz,kbcpn,fzsl,mjzrj,bmj,mksmf,gptv,kgkrhg
*/

pub fn day21() {
    let contents = std::fs::read_to_string("input21.txt").expect("Failed to read file");
    let foods = parse_foods(contents);
    let possible_translations = get_possible_translations(&foods);
    let translations = get_translations(possible_translations.iter(), Ingredients::new()).unwrap();

    println!(
        "Day 21, part 1: {}",
        part1(&foods, translations.values().cloned().collect())
    );
    println!("Day 21, part 2: {:?}", part2(translations));
}
