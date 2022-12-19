use std::collections::HashMap;
use text_io::scan;

#[derive(Default, Clone, Copy)]
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
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

impl Inventory {
    fn can_afford(&self, cost: Cost) -> bool {
        self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
    }

    fn need_ore(&self, blueprint: &Blueprint) -> bool {
        if self.ore_robot < blueprint.geode.ore {
            return true;
        }
        if self.obsidian_robot >= blueprint.geode.obsidian {
            return false;
        }
        if self.ore_robot < blueprint.obsidian.ore {
            return true;
        }
        if self.clay_robot >= blueprint.obsidian.clay {
            return false;
        }
        if self.ore_robot < blueprint.clay.ore {
            return true;
        }

        false
    }

    fn need_clay(&self, blueprint: &Blueprint) -> bool {
        self.need_obsidian(blueprint) && self.clay_robot < blueprint.obsidian.clay
    }

    fn need_obsidian(&self, blueprint: &Blueprint) -> bool {
        self.obsidian_robot < blueprint.geode.obsidian
    }

    fn spend(&mut self, cost: Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }

    fn produce(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
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

    // Trying not to let the memoization table grow too big and start hurting
    // more than it's helping.
    if minutes >= 5 {
        if let Some(score) = memoize.get(&(inv, minutes)) {
            return *score;
        }
    }

    let mut score = 0;

    if inv.can_afford(blueprint.ore) && inv.need_ore(blueprint) {
        let mut new_inv = inv;
        new_inv.spend(blueprint.ore);
        new_inv.produce();
        new_inv.ore_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if inv.can_afford(blueprint.clay) && inv.need_clay(blueprint) {
        let mut new_inv = inv;
        new_inv.spend(blueprint.clay);
        new_inv.produce();
        new_inv.clay_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if inv.can_afford(blueprint.obsidian) && inv.need_obsidian(blueprint) {
        let mut new_inv = inv;
        new_inv.spend(blueprint.obsidian);
        new_inv.produce();
        new_inv.obsidian_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    if inv.can_afford(blueprint.geode) {
        let mut new_inv = inv;
        new_inv.spend(blueprint.geode);
        new_inv.produce();
        new_inv.geode_robot += 1;
        score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));
    }

    let mut new_inv = inv;
    new_inv.produce();
    score = score.max(simulate(memoize, blueprint, new_inv, minutes - 1));

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

    let quality_sum = blueprints.iter()
        .map(|b| {
            memoize_table.clear();
            b.id * simulate(&mut memoize_table, b, inv, 24)
        })
        .sum();

    let product = blueprints[..3].iter()
        .map(|b| {
            memoize_table.clear();
            simulate(&mut memoize_table, b, inv, 32)
        })
        .product();

    (quality_sum, product)
}

#[cfg(test)]
#[test]
#[ignore] // this takes 5 minutes but the real problem takes 30 seconds...
fn example() {
    let input =
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    let output = solve(input);
    assert_eq!(output.0, 33);
    assert_eq!(output.1, 56 * 62);
}
