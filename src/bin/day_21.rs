use adventofcode2020::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

fn main() {
    let mut allergen_map = HashMap::<String, HashSet<String>>::new();
    let mut ingredient_occurrences = HashMap::<String, usize>::new();

    lines_from_file("input/day_21.txt", |line| {
        let contains = line.find(" (contains ").unwrap();
        let ingredients = line[..contains]
            .split(' ')
            .map(String::from)
            .collect::<HashSet<_>>();
        let allergens = line[(contains + 11)..(line.len() - 1)]
            .split(", ")
            .map(String::from)
            .collect::<Vec<_>>();

        // For each allergen, find the set intersection of all the ingredients
        // that could contain that allergen.
        for allergen in allergens.iter() {
            match allergen_map.entry(allergen.clone()) {
                Entry::Vacant(entry) => { entry.insert(ingredients.clone()); },
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry.get()
                        .intersection(&ingredients)
                        .map(|i| i.clone())
                        .collect();
                }
            }
        }

        // Keep track of the number of times each ingredient appears.
        for ingredient in ingredients.iter() {
            *ingredient_occurrences.entry(ingredient.clone()).or_insert(0) += 1;
        }
    });

    // Remove the occurrences for ingredients that could contain an allergen. We
    // only care about ingredients that couldn't possibly contain an allergen.
    for (_, ingredients) in allergen_map.iter() {
        for ingredient in ingredients.iter() {
            ingredient_occurrences.remove(ingredient);
        }
    }

    // Sum of all occurrences
    let mut sum = 0;
    for (_, occurrences) in ingredient_occurrences.iter() {
        sum += *occurrences;
    }

    println!("Part one: {}", sum);

    let mut danger_list = Vec::<(String, String)>::new();

    while !allergen_map.is_empty() {
        // Find allergens that have one ingredient. We know that this is "the"
        // ingredient for that allergen.
        for (allergen, ingredient_set) in allergen_map.iter() {
            if ingredient_set.len() == 1 {
                let ingredient = ingredient_set.iter().next().unwrap();
                danger_list.push((allergen.clone(), ingredient.clone()));
            }
        }

        // Remove the allergens that we've found the ingredient for.
        for (allergen, _) in danger_list.iter() {
            allergen_map.remove(allergen);
        }

        // Remove ingredients that we know are the ingredients for another
        // allergen.
        for (_, ingredient_set) in allergen_map.iter_mut() {
            for (_, ingredient) in danger_list.iter() {
                ingredient_set.remove(ingredient);
            }
        }
    }

    // Sort by allergen.
    danger_list.sort_by_key(|(allergen, _)| allergen.clone());

    // Join the ingredient list with commas.
    let mut canonical_danger_list = String::new();
    for (_, ingredient) in danger_list.iter() {
        canonical_danger_list += ingredient;
        canonical_danger_list += ",";
    }
    canonical_danger_list.pop();

    println!("Part two: {}", canonical_danger_list);
}
