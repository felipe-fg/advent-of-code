use super::utils;
use itertools::join;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl AsRef<Self> for Food {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub fn run() {
    let lines: Vec<String> = utils::read_lines("inputs/day21.txt", true).collect();

    let foods = parse_foods(&lines);

    let allergens = find_allergen_ingredients(&foods);

    let safe_count = count_safe_ingredients(&foods, &allergens);

    let dangerous_list = dangerous_ingredients(&allergens);

    println!("{:?}", safe_count);
    println!("{:?}", dangerous_list);
}

fn parse_foods(lines: &[String]) -> Vec<Food> {
    let re = Regex::new(r"(?P<ingredients>.+) [(]contains (?P<allergens>.+)[)]").expect("regex");

    lines
        .iter()
        .map(|line| {
            re.captures(line)
                .map(|caps| {
                    let ingredients = caps["ingredients"]
                        .split_whitespace()
                        .map(|value| value.to_string())
                        .collect();

                    let allergens = caps["allergens"]
                        .split(", ")
                        .map(|value| value.to_string())
                        .collect();

                    Food {
                        ingredients,
                        allergens,
                    }
                })
                .expect("captures")
        })
        .collect()
}

fn find_allergen_ingredients<T: AsRef<Food>>(foods: &[T]) -> Vec<(String, String)> {
    let mut allergen_names = vec![];
    let mut remaining_allergens = get_allergens(foods);
    let mut foods: Vec<Food> = foods.iter().map(|food| food.as_ref().clone()).collect();

    while !remaining_allergens.is_empty() {
        if let Some((allergen, ingredient)) = next_allergen_match(&foods, &remaining_allergens) {
            foods = remove_allergen_ingredient(&foods, &allergen, &ingredient);

            remaining_allergens.retain(|x| x != &allergen);
            allergen_names.push((allergen, ingredient));
        }
    }

    allergen_names.sort_unstable_by_key(|(allergen, _)| allergen.to_string());

    allergen_names
}

fn get_allergens<T: AsRef<Food>>(foods: &[T]) -> HashSet<String> {
    foods
        .iter()
        .flat_map(|food| &food.as_ref().allergens)
        .map(|allergen| allergen.to_string())
        .collect()
}

fn next_allergen_match<T, U>(foods: &[T], allergens: &HashSet<U>) -> Option<(String, String)>
where
    T: AsRef<Food>,
    U: AsRef<str>,
{
    allergens
        .iter()
        .map(|allergen| {
            let allergen = allergen.as_ref().to_string();

            let allergen_foods: Vec<&Food> = foods
                .iter()
                .map(|food| food.as_ref())
                .filter(|food| food.allergens.contains(&allergen))
                .collect();

            let allergen_ingredients = find_common_ingredients(&allergen_foods);

            (allergen, allergen_ingredients)
        })
        .find(|(_, ingredients)| ingredients.len() == 1)
        .map(|(allergen, ingredients)| (allergen, ingredients[0].to_string()))
}

fn find_common_ingredients<T: AsRef<Food>>(foods: &[T]) -> Vec<String> {
    let mut ingredients = HashMap::new();

    foods
        .iter()
        .flat_map(|food| &food.as_ref().ingredients)
        .for_each(|ingredient| {
            ingredients
                .entry(ingredient)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    ingredients
        .iter()
        .filter(|(_, count)| count == &&foods.len())
        .map(|(ingredient, _)| ingredient.to_string())
        .collect()
}

fn remove_allergen_ingredient<T: AsRef<Food>>(
    foods: &[T],
    allergen: &str,
    ingredient: &str,
) -> Vec<Food> {
    foods
        .iter()
        .map(|food| {
            let mut food = food.as_ref().clone();
            food.ingredients.retain(|x| x != ingredient);
            food.allergens.retain(|x| x != allergen);

            food
        })
        .collect()
}

fn count_safe_ingredients<T, U>(foods: &[T], allergens: &[(U, U)]) -> usize
where
    T: AsRef<Food>,
    U: AsRef<str>,
{
    foods
        .iter()
        .flat_map(|food| &food.as_ref().ingredients)
        .filter(|ingredient| {
            !allergens
                .iter()
                .any(|(_, allergen_ingredient)| &allergen_ingredient.as_ref() == ingredient)
        })
        .count()
}

fn dangerous_ingredients<T: AsRef<str>>(allergens: &[(T, T)]) -> String {
    let iter = allergens.iter().map(|(_, ingredient)| ingredient.as_ref());

    join(iter, ",")
}
