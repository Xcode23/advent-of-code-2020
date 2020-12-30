use std::collections::{BTreeMap, HashMap, HashSet};

use crate::input;

static INPUT: &str = input::_INPUT;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

pub fn allergen_part1() -> i64 {
    let data: Vec<Recipe> = INPUT.lines().map(|line| {
        let mut parts = line.split(" (contains");
        let ingredients = parts.next().unwrap().trim()
            .split(" ").map(|ingredient| ingredient.trim().to_string()).collect();
        let allergens = parts.next().unwrap().strip_suffix(")").unwrap()
            .trim().split(", ").map(|allergen| allergen.trim().to_string()).collect();
        Recipe {
            ingredients,
            allergens
        }
    }).collect();
    let mut possible_associations: HashMap<String, HashSet<String>> = HashMap::new();
    for recipe in data.iter() {
        for allergen in recipe.allergens.iter().cloned() {
            let value = possible_associations.entry(allergen)
                .or_insert(recipe.ingredients.clone());
            *value = value.intersection(&recipe.ingredients).cloned().collect();
        }
    }

    let mut associations: HashMap<String, String> = HashMap::new();
    loop {
        let old_len = associations.len();
        let mut allergic_ingredients = HashSet::new();
        possible_associations.iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .for_each(|(allergen, ingredients)| {
                let allergic_ingredient = ingredients.into_iter().next().unwrap();
                allergic_ingredients.insert(allergic_ingredient.clone());
                associations.insert(allergen.clone(), allergic_ingredient.clone());
            });
        possible_associations.retain(|_, ingredients| ingredients.len() != 1);
        possible_associations.iter_mut().for_each(|(_, ingredients)| {
            ingredients.retain(|ingredient| !allergic_ingredients.contains(ingredient))
        });
        if old_len == associations.len() { break }
    }

    let ingredients_to_test: HashSet<String> = data.iter().map(|recipe| recipe.ingredients.iter().cloned()).flatten().filter(|ingredient| !associations.values().collect::<Vec<_>>().contains(&ingredient)).collect();

    data.into_iter().fold(0 as i64, |acc, recipe | {
        acc + ingredients_to_test.intersection(&recipe.ingredients).count() as i64
    })
}

pub fn allergen_part2() -> i64 {
    let data: Vec<Recipe> = INPUT.lines().map(|line| {
        let mut parts = line.split(" (contains");
        let ingredients = parts.next().unwrap().trim()
            .split(" ").map(|ingredient| ingredient.trim().to_string()).collect();
        let allergens = parts.next().unwrap().strip_suffix(")").unwrap()
            .trim().split(", ").map(|allergen| allergen.trim().to_string()).collect();
        Recipe {
            ingredients,
            allergens
        }
    }).collect();
    let mut possible_associations: HashMap<String, HashSet<String>> = HashMap::new();
    for recipe in data.iter() {
        for allergen in recipe.allergens.iter().cloned() {
            let value = possible_associations.entry(allergen)
                .or_insert(recipe.ingredients.clone());
            *value = value.intersection(&recipe.ingredients).cloned().collect();
        }
    }

    let mut associations: BTreeMap<String, String> = BTreeMap::new();
    loop {
        let old_len = associations.len();
        let mut allergic_ingredients = HashSet::new();
        possible_associations.iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .for_each(|(allergen, ingredients)| {
                let allergic_ingredient = ingredients.into_iter().next().unwrap();
                allergic_ingredients.insert(allergic_ingredient.clone());
                associations.insert(allergen.clone(), allergic_ingredient.clone());
            });
        possible_associations.retain(|_, ingredients| ingredients.len() != 1);
        possible_associations.iter_mut().for_each(|(_, ingredients)| {
            ingredients.retain(|ingredient| !allergic_ingredients.contains(ingredient))
        });
        if old_len == associations.len() { break }
    }

    println!("{:?}", associations.values());
    0
}