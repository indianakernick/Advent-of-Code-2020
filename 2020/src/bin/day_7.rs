use regex::Regex;
use adventofcode2020::lines_from_file;
use std::collections::hash_map::HashMap;

type BagsMap = HashMap::<String, Vec<(usize, String)>>;

fn parse_input() -> BagsMap {
    let mut map = BagsMap::new();

    let no_bags = Regex::new(r"([\w ]+) bags contain no other bags").unwrap();
    let one_bag = Regex::new(r"([\w ]+) bags contain (\d+) ([\w ]+) bags?").unwrap();
    let extra_bags = Regex::new(r", (\d) ([\w ]+) bags?").unwrap();

    lines_from_file("input/day_7.txt", |line| {
        // No bags
        if let Some(no_cap) = no_bags.captures(line) {
            map.insert(no_cap[1].to_owned(), Vec::new());
            return;
        }

        // At lease one bag
        let one_cap = one_bag.captures(line).unwrap();
        let outer = one_cap[1].to_owned();
        let mut inner_list = Vec::new();
        inner_list.push((one_cap[2].parse().unwrap(), one_cap[3].to_owned()));

        // More than one bag
        for ex_cap in extra_bags.captures_iter(line) {
            inner_list.push((ex_cap[1].parse().unwrap(), ex_cap[2].to_owned()));
        }

        map.insert(outer, inner_list);
    });

    map
}

// Determine whether outer contains inner (possibly indirectly)
fn contains_bag(map: &BagsMap, outer: &str, inner: &str) -> bool {
    map[outer]
        .iter()
        .any(|(_, bag)| bag == inner || contains_bag(map, bag, inner))
}

// Part one.
// Find the number of unique bag colors that contain the given bag.
fn count_bags_containing(map: &BagsMap, bag: &str) -> usize {
    map
        .iter()
        .filter(|(outer, _)| contains_bag(map, outer, bag))
        .count()
}

// Part two.
// Find the number of bags that are contained by the given bag.
fn count_contained_bags(map: &BagsMap, outer: &str) -> usize {
    map[outer]
        .iter()
        .fold(1, |sum, (count, bag)| sum + count * count_contained_bags(&map, &bag))
}

fn main() {
    let map = parse_input();
    println!("Part one: {}", count_bags_containing(&map, "shiny gold"));
    println!("Part two: {}", count_contained_bags(&map, "shiny gold") - 1);
}
