use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

struct Blueprint {
    id: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: (usize, usize),
    geode_robot: (usize, usize),
}

fn recurse(
    ore: usize,
    clay: usize,
    obsidian: usize,
    robots: (usize, usize, usize, usize),
    remaining: usize,
    blueprint: &Blueprint,
    cache: &mut HashSet<(usize, usize, usize, (usize, usize, usize, usize), usize)>,
) -> usize {
    if remaining == 0 {
        return 0;
    }
    if cache.contains(&(ore, clay, obsidian, robots, remaining)) {
        return 0;
    }
    cache.insert((ore, clay, obsidian, robots, remaining));
    let mut geodes = 0;
    if ore >= blueprint.geode_robot.0 && obsidian >= blueprint.geode_robot.1 {
        geodes = geodes.max(recurse(
            ore + robots.0 - blueprint.geode_robot.0,
            clay + robots.1,
            obsidian + robots.2 - blueprint.geode_robot.1,
            (robots.0, robots.1, robots.2, robots.3 + 1),
            remaining - 1,
            blueprint,
            cache,
        ));
        return geodes + robots.3;
    }
    if ore >= blueprint.obsidian_robot.0
        && clay >= blueprint.obsidian_robot.1
        && robots.2 < blueprint.geode_robot.1
    {
        geodes = geodes.max(recurse(
            ore + robots.0 - blueprint.obsidian_robot.0,
            clay + robots.1 - blueprint.obsidian_robot.1,
            obsidian + robots.2,
            (robots.0, robots.1, robots.2 + 1, robots.3),
            remaining - 1,
            blueprint,
            cache,
        ));
    }
    if ore >= blueprint.clay_robot && robots.1 < blueprint.obsidian_robot.1 {
        geodes = geodes.max(recurse(
            ore + robots.0 - blueprint.clay_robot,
            clay + robots.1,
            obsidian + robots.2,
            (robots.0, robots.1 + 1, robots.2, robots.3),
            remaining - 1,
            blueprint,
            cache,
        ));
    }
    if ore >= blueprint.ore_robot
        && (robots.0 < blueprint.ore_robot
            || robots.0 < blueprint.clay_robot
            || robots.0 < blueprint.obsidian_robot.0
            || robots.0 < blueprint.geode_robot.0)
    {
        geodes = geodes.max(recurse(
            ore + robots.0 - blueprint.ore_robot,
            clay + robots.1,
            obsidian + robots.2,
            (robots.0 + 1, robots.1, robots.2, robots.3),
            remaining - 1,
            blueprint,
            cache,
        ));
    }
    geodes = geodes.max(recurse(
        ore + robots.0,
        clay + robots.1,
        obsidian + robots.2,
        robots,
        remaining - 1,
        blueprint,
        cache,
    ));
    geodes + robots.3
}

fn main() {
    let mut blueprints = Vec::new();

    let regex = Regex::new(r"\d+").unwrap();
    for line in include_str!("input.txt").lines() {
        let mut matches = regex
            .find_iter(line)
            .map(|num| num.as_str().parse().unwrap());

        blueprints.push(Blueprint {
            id: matches.next().unwrap(),
            ore_robot: matches.next().unwrap(),
            clay_robot: matches.next().unwrap(),
            obsidian_robot: matches.next_tuple().unwrap(),
            geode_robot: matches.next_tuple().unwrap(),
        })
    }
    print!(
        "{}",
        blueprints
            .iter()
            .take(3)
            .map(|blueprint| recurse(0, 0, 0, (1, 0, 0, 0), 32, blueprint, &mut HashSet::new()))
            .product::<usize>()
    );
}
