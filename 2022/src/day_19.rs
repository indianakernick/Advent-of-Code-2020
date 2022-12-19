use std::collections::HashMap;

use text_io::scan;

#[derive(Default)]
struct Cost {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

struct Blueprint {
    id: u16,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Inventory {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_robot: u16,
    clay_robot: u16,
    obsidian_robot: u16,
    geode_robot: u16,
}

fn can_afford(inv: &Inventory, cost: &Cost) -> bool {
    inv.ore >= cost.ore
    && inv.clay >= cost.clay
    && inv.obsidian >= cost.obsidian
}

fn spend(inv: &mut Inventory, cost: &Cost) {
    inv.ore -= cost.ore;
    inv.clay -= cost.clay;
    inv.obsidian -= cost.obsidian;
}

fn produce(inv: &mut Inventory) {
    inv.ore += inv.ore_robot;
    inv.clay += inv.clay_robot;
    inv.obsidian += inv.obsidian_robot;
    inv.geode += inv.geode_robot;
}

fn simulate(
    memoize: &mut HashMap::<(Inventory, u16), u16>,
    blueprint: &Blueprint,
    inv: Inventory,
    minutes: u16,
) -> u16 {
    if minutes == 0 {
        return inv.geode;
    }

    if minutes >= 5 {
        if let Some(score) = memoize.get(&(inv.clone(), minutes)) {
            return *score;
        }
    }

    let mut score = 0;

    if can_afford(&inv, &blueprint.ore) {
        let mut new_inv = inv.clone();
        spend(&mut new_inv, &blueprint.ore);
        produce(&mut new_inv);
        new_inv.ore_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if can_afford(&inv, &blueprint.clay) {
        let mut new_inv = inv.clone();
        spend(&mut new_inv, &blueprint.clay);
        produce(&mut new_inv);
        new_inv.clay_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if can_afford(&inv, &blueprint.geode) {
        let mut new_inv = inv.clone();
        spend(&mut new_inv, &blueprint.geode);
        produce(&mut new_inv);
        new_inv.geode_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    } else if can_afford(&inv, &blueprint.obsidian) {
        let mut new_inv = inv.clone();
        spend(&mut new_inv, &blueprint.obsidian);
        produce(&mut new_inv);
        new_inv.obsidian_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    } else {
        let mut new_inv = inv.clone();
        produce(&mut new_inv);
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if minutes >= 5 {
        memoize.insert((inv, minutes), score);
    }
    score
}

pub fn solve(input: &str) -> (u16, u16) {
    let mut blueprints = Vec::<Blueprint>::new();

    for line in input.lines() {
        let mut line = line.bytes();
        let i: u16;
        let or_or: u16;
        let cl_or: u16;
        let ob_or: u16;
        let ob_cl: u16;
        let ge_or: u16;
        let ge_ob: u16;
        scan!(
            line => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            i, or_or, cl_or, ob_or, ob_cl, ge_or, ge_ob
        );
        blueprints.push(Blueprint {
            id: i,
            ore: Cost { ore: or_or, ..Default::default() },
            clay: Cost { ore: cl_or, ..Default::default() },
            obsidian: Cost { ore: ob_or, clay: ob_cl, ..Default::default() },
            geode: Cost { ore: ge_or, obsidian: ge_ob, ..Default::default() }
        });
    }

    let inv = Inventory {
        ore_robot: 1,
        ..Default::default()
    };
    let mut memoize_table = HashMap::new();
    /*let quality_sum = blueprints.iter()
        .map(|b| {
            println!("Processing blueprint: {}", b.id);
            memoize_table.clear();
            b.id * simulate(&mut memoize_table, b, inv.clone(), 24)
        })
        .sum();*/
        let quality_sum = 0;

    let product = blueprints.iter().take(3).map(|b| {
        println!("Processing blueprint: {}", b.id);
        memoize_table.clear();
        simulate(&mut memoize_table, b, inv.clone(), 32)
    })
    .product();

    (quality_sum, product)
}
