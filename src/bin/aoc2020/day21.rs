use std::collections::HashMap;

use itertools::Itertools;

fn parse(input: &str) -> HashMap<Vec<String>, Vec<String>> {
    let mut map = HashMap::new();

    for l in input.lines() {
        let (ingredients, allergens) = l.split_once(" (").unwrap();
        let ingredients: Vec<String> = ingredients.split(' ').map(|x| x.to_owned()).collect();

        let allergens = &allergens[9..allergens.len() - 1];
        let allergens: Vec<String> = allergens.split(", ").map(|x| x.to_owned()).collect();

        map.insert(ingredients, allergens);
    }

    map
}

fn unsafe_ingredients(allergen: &str, foods: &HashMap<Vec<String>, Vec<String>>) -> Vec<String> {
    let has_allergen: Vec<&Vec<String>> = foods
        .iter()
        .filter(|(_, allergens)| allergens.iter().any(|a| a == allergen))
        .map(|(ingredients, _)| ingredients)
        .collect();

    let mut shared: HashMap<&String, usize> = HashMap::new();
    for ingredients in &has_allergen {
        for i in *ingredients {
            *shared.entry(i).or_insert(0) += 1;
        }
    }

    shared
        .into_iter()
        .filter(|(_, count)| *count == has_allergen.len())
        .map(|(i, _)| i.clone())
        .collect()
}

fn part1(input: &str) -> usize {
    let foods = parse(input);

    let all_ingredients: Vec<&String> = foods.keys().flatten().collect();
    let all_allergens: Vec<&String> = foods.values().flatten().unique().collect();

    let unsafe_set: Vec<String> = all_allergens
        .iter()
        .flat_map(|a| unsafe_ingredients(a, &foods))
        .unique()
        .collect();

    all_ingredients
        .iter()
        .filter(|i| !unsafe_set.contains(i))
        .count()
}

fn part2(input: &str) -> String {
    let foods = parse(input);

    let mut candidates: Vec<(String, Vec<String>)> = foods
        .values()
        .flatten()
        .unique()
        .map(|a| (a.clone(), unsafe_ingredients(a, &foods)))
        .collect();

    let mut resolved: Vec<(String, String)> = Vec::new();

    while !candidates.is_empty() {
        let i = candidates.iter().position(|(_, is)| is.len() == 1).unwrap();
        let (allergen, ingredients) = candidates.remove(i);
        let ingredient = ingredients[0].clone();
        resolved.push((allergen, ingredient.clone()));

        for (_, is) in candidates.iter_mut() {
            is.retain(|i| *i != ingredient);
        }
    }

    resolved.sort_by(|a, b| a.0.cmp(&b.0));
    resolved.into_iter().map(|(_, i)| i).join(",")
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/21.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), "mxmxvkd,sqjhc,fvjkl");
    }
}
