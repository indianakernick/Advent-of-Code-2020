use adventofcode2020::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut allergen_map = HashMap::<String, HashSet<String>>::new();
    let mut ingredient_occurrences = HashMap::<String, usize>::new();

    lines_from_file("input/day_21.txt", |line| {
        let contains = line.find(" (contains ").unwrap();
        let ingredients = line[..contains].split(' ').map(String::from).collect::<HashSet<_>>();
        let allergens = line[(contains + 11)..(line.len() - 1)].split(", ").map(String::from).collect::<Vec<_>>();

        for allergen in allergens.iter() {
            if allergen_map.contains_key(allergen) {
                *allergen_map.get_mut(allergen).unwrap() = allergen_map[allergen].intersection(&ingredients).map(|i|i.clone()).collect();
            } else {
                allergen_map.insert(allergen.clone(), ingredients.clone());
            }
        }

        for ingredient in ingredients.iter() {
            *ingredient_occurrences.entry(ingredient.clone()).or_insert(0) += 1;
        }
    });

    for (_, ingredients) in allergen_map.iter() {
        for ingredient in ingredients.iter() {
            ingredient_occurrences.remove(ingredient);
        }
    }

    let mut sum = 0;
    for (_, occurrences) in ingredient_occurrences.iter() {
        sum += *occurrences;
    }

    println!("Part one: {}", sum);

    let mut danger_list = Vec::<(String, String)>::new();

    loop {
        let mut need_to_remove = false;
        for (allergen, ingredient_set) in allergen_map.iter() {
            if ingredient_set.len() == 1 {
                let ingredient = ingredient_set.iter().next().unwrap();
                danger_list.push((allergen.clone(), ingredient.clone()));
            } else {
                need_to_remove = true;
            }
        }

        if !need_to_remove {
            break;
        }

        for (allergen, _) in danger_list.iter() {
            allergen_map.remove(allergen);
        }

        for (_, ingredient_set) in allergen_map.iter_mut() {
            for (_, ingredient) in danger_list.iter() {
                ingredient_set.remove(ingredient);
            }
        }
    }

    danger_list.sort_by_key(|(allergen, _)| allergen.clone());

    let mut canonical_danger_list = String::new();
    for (_, ingredient) in danger_list.iter() {
        canonical_danger_list += ingredient;
        canonical_danger_list += ",";
    }
    canonical_danger_list.pop();

    println!("Part two: {}", canonical_danger_list);
}
