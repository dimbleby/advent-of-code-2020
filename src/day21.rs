use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Recipe {
    fn new(ingredients: HashSet<String>, allergens: HashSet<String>) -> Self {
        Self {
            ingredients,
            allergens,
        }
    }
}

impl FromStr for Recipe {
    type Err = ParseError;

    // I removed all the brackets to make this simpler.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split(" contains ");
        let ingredients: HashSet<String> = sections
            .next()
            .ok_or(ParseError)?
            .split_whitespace()
            .map(String::from)
            .collect();
        let allergens: HashSet<String> = sections
            .next()
            .ok_or(ParseError)?
            .split(", ")
            .map(String::from)
            .collect();

        let recipe = Recipe::new(ingredients, allergens);
        Ok(recipe)
    }
}

pub(crate) fn day21() {
    let recipes: Vec<Recipe> = std::fs::read_to_string("data/day21.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut possibilities: HashMap<&str, HashSet<&str>> = HashMap::new();
    for recipe in &recipes {
        for allergen in &recipe.allergens {
            let ingredients = recipe.ingredients.iter().map(|i| i.as_str()).collect();
            let so_far = possibilities.get(allergen.as_str()).unwrap_or(&ingredients);
            let updated = so_far.intersection(&ingredients).cloned().collect();
            possibilities.insert(allergen, updated);
        }
    }
    let danger_ingredients: HashSet<&str> = possibilities.values().flatten().cloned().collect();

    let safe_appearances = recipes
        .iter()
        .map(|recipe| recipe.ingredients.iter())
        .flatten()
        .filter(|ingredient| !danger_ingredients.contains(ingredient.as_str()))
        .count();
    println!("Part one answer is {}", safe_appearances);

    let mut matching: HashMap<&str, &str> = HashMap::new();
    while let Some((&allergen, ingredients)) = possibilities
        .iter()
        .find(|(_, candidates)| candidates.len() == 1)
    {
        let ingredient = <&str>::clone(ingredients.iter().next().unwrap());
        matching.insert(ingredient, allergen);
        possibilities.remove(allergen);
        for candidates in possibilities.values_mut() {
            candidates.remove(&ingredient);
        }
    }
    let mut answer: Vec<&str> = matching.keys().cloned().collect();
    answer.sort_by(|a, b| matching[a].cmp(matching[b]));
    println!("Part two answer is {}", answer.join(","));
}
