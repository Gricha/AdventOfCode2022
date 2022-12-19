use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use crate::utils::read_input;

#[derive(Debug)]
struct Blueprint {
    idx: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot_ore: i32,
    obsidian_robot_clay: i32,
    geode_robot_ore: i32,
    geode_robot_obsidian: i32,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

impl Resources {
    fn can_afford_robot(&self, blueprint: &Blueprint, robot: Robot) -> bool {
        match robot {
            Robot::Ore => self.ore >= blueprint.ore_robot,
            Robot::Clay => self.ore >= blueprint.clay_robot,
            Robot::Obsidian => {
                self.ore >= blueprint.obsidian_robot_ore
                    && self.clay >= blueprint.obsidian_robot_clay
            }
            Robot::Geode => {
                self.ore >= blueprint.geode_robot_ore
                    && self.obsidian >= blueprint.geode_robot_obsidian
            }
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct State {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    resources_in_inventory: Resources,
    time: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .resources_in_inventory
            .geodes
            .cmp(&self.resources_in_inventory.geodes)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run_easy() {
    let regex = Regex::new(r"Blueprint (.*?): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    // let captures_left = regex.captures(left).unwrap();
    let input: i32 = read_input("inputs/day19.txt")
        .into_iter()
        .map(|val| {
            let captures = regex.captures(&val).unwrap();
            let (
                idx,
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obsidian_robot_cost_ore,
                obsidian_robot_cost_clay,
                geode_robot_cost_ore,
                geode_robot_cost_obsidian,
            ) = (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(7).unwrap().as_str().parse::<i32>().unwrap(),
            );

            Blueprint {
                idx,
                ore_robot: ore_robot_cost_ore,
                clay_robot: clay_robot_cost_ore,
                obsidian_robot_ore: obsidian_robot_cost_ore,
                obsidian_robot_clay: obsidian_robot_cost_clay,
                geode_robot_ore: geode_robot_cost_ore,
                geode_robot_obsidian: geode_robot_cost_obsidian,
            }
        })
        .collect_vec()
        .par_iter()
        .map(|blueprint| {
            let mut cache = HashSet::new();

            let mut queue = VecDeque::<State>::new();

            let max_needed_ores = std::cmp::max(
                std::cmp::max(
                    std::cmp::max(blueprint.clay_robot, blueprint.ore_robot),
                    blueprint.geode_robot_ore,
                ),
                blueprint.obsidian_robot_ore,
            );

            queue.push_back(State {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                resources_in_inventory: Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                time: 0,
            });

            let mut maximum_geodes = 0;

            while !queue.is_empty() {
                let state = queue.pop_front().unwrap();

                maximum_geodes = std::cmp::max(maximum_geodes, state.resources_in_inventory.geodes);

                if state.time == 24 {
                    continue;
                }

                if cache.contains(&state) {
                    continue;
                }

                cache.insert(state);

                let ore = state.resources_in_inventory.ore + state.ore_robots;
                let clay = state.resources_in_inventory.clay + state.clay_robots;
                let obsidian = state.resources_in_inventory.obsidian + state.obsidian_robots;
                let geodes = state.resources_in_inventory.geodes + state.geode_robots;

                let resources = Resources {
                    ore,
                    clay,
                    obsidian,
                    geodes,
                };

                assert!(resources.ore >= 0);
                assert!(resources.clay >= 0);
                assert!(resources.obsidian >= 0);
                assert!(resources.geodes >= 0);

                queue.push_back(State {
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                    resources_in_inventory: resources,
                    time: state.time + 1,
                });

                // if state.time < 24 {
                if resources.can_afford_robot(blueprint, Robot::Ore)
                    && state.ore_robots < max_needed_ores
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots + 1,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.ore_robot - 1,
                            clay: resources.clay,
                            obsidian: resources.obsidian,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    });
                }

                if resources.can_afford_robot(blueprint, Robot::Clay)
                    && state.clay_robots < blueprint.obsidian_robot_clay
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots + 1,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.clay_robot,
                            clay: resources.clay - 1,
                            obsidian: resources.obsidian,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    })
                }

                if resources.can_afford_robot(blueprint, Robot::Obsidian)
                    && state.obsidian_robots < blueprint.geode_robot_obsidian
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots + 1,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.obsidian_robot_ore,
                            clay: resources.clay - blueprint.obsidian_robot_clay,
                            obsidian: resources.obsidian - 1,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    })
                }

                if resources.can_afford_robot(blueprint, Robot::Geode) {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots + 1,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.geode_robot_ore,
                            clay: resources.clay,
                            obsidian: resources.obsidian - blueprint.geode_robot_obsidian,
                            geodes: resources.geodes - 1,
                        },
                        time: state.time + 1,
                    })
                }
            }
            // }

            println!("{} {}", blueprint.idx, maximum_geodes);

            maximum_geodes * blueprint.idx
        })
        .sum();

    dbg!(input);
}

pub fn run_hard() {
    let regex = Regex::new(r"Blueprint (.*?): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    // let captures_left = regex.captures(left).unwrap();
    let input: i32 = read_input("inputs/day19.txt")
        .into_iter()
        .take(3)
        .map(|val| {
            let captures = regex.captures(&val).unwrap();
            let (
                idx,
                ore_robot_cost_ore,
                clay_robot_cost_ore,
                obsidian_robot_cost_ore,
                obsidian_robot_cost_clay,
                geode_robot_cost_ore,
                geode_robot_cost_obsidian,
            ) = (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(7).unwrap().as_str().parse::<i32>().unwrap(),
            );

            Blueprint {
                idx,
                ore_robot: ore_robot_cost_ore,
                clay_robot: clay_robot_cost_ore,
                obsidian_robot_ore: obsidian_robot_cost_ore,
                obsidian_robot_clay: obsidian_robot_cost_clay,
                geode_robot_ore: geode_robot_cost_ore,
                geode_robot_obsidian: geode_robot_cost_obsidian,
            }
        })
        .collect_vec()
        .par_iter()
        .map(|blueprint| {
            let mut cache = HashSet::new();

            let mut queue = VecDeque::<State>::new();

            let max_needed_ores = std::cmp::max(
                std::cmp::max(
                    std::cmp::max(blueprint.clay_robot, blueprint.ore_robot),
                    blueprint.geode_robot_ore,
                ),
                blueprint.obsidian_robot_ore,
            );

            queue.push_back(State {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                resources_in_inventory: Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                time: 0,
            });

            let mut maximum_geodes = 0;

            while !queue.is_empty() {
                let state = queue.pop_front().unwrap();

                maximum_geodes = std::cmp::max(maximum_geodes, state.resources_in_inventory.geodes);

                if state.time == 32 {
                    continue;
                }

                let ore = state.resources_in_inventory.ore + state.ore_robots;
                let clay = state.resources_in_inventory.clay + state.clay_robots;
                let obsidian = state.resources_in_inventory.obsidian + state.obsidian_robots;
                let geodes = state.resources_in_inventory.geodes + state.geode_robots;

                let mut resources = Resources {
                    ore,
                    clay,
                    obsidian,
                    geodes,
                };

                assert!(resources.ore >= 0);
                assert!(resources.clay >= 0);
                assert!(resources.obsidian >= 0);
                assert!(resources.geodes >= 0);

                let remaining_time = 32 - state.time;
                resources.ore = std::cmp::min(resources.ore, max_needed_ores * remaining_time);
                resources.clay = std::cmp::min(
                    resources.clay,
                    blueprint.obsidian_robot_clay * remaining_time,
                );
                resources.obsidian = std::cmp::min(
                    resources.obsidian,
                    blueprint.geode_robot_obsidian * remaining_time,
                );

                if cache.contains(&state) {
                    continue;
                }

                cache.insert(state);

                if !(resources.can_afford_robot(blueprint, Robot::Clay)
                    && resources.can_afford_robot(blueprint, Robot::Ore)
                    && resources.can_afford_robot(blueprint, Robot::Obsidian)
                    && resources.can_afford_robot(blueprint, Robot::Geode))
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: resources,
                        time: state.time + 1,
                    });
                }

                // if state.time < 24 {
                if resources.can_afford_robot(blueprint, Robot::Ore)
                    && state.ore_robots < max_needed_ores
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots + 1,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.ore_robot - 1,
                            clay: resources.clay,
                            obsidian: resources.obsidian,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    });
                }

                if resources.can_afford_robot(blueprint, Robot::Clay)
                    && state.clay_robots < blueprint.obsidian_robot_clay
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots + 1,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.clay_robot,
                            clay: resources.clay - 1,
                            obsidian: resources.obsidian,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    })
                }

                if resources.can_afford_robot(blueprint, Robot::Obsidian)
                    && state.obsidian_robots < blueprint.geode_robot_obsidian
                {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots + 1,
                        geode_robots: state.geode_robots,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.obsidian_robot_ore,
                            clay: resources.clay - blueprint.obsidian_robot_clay,
                            obsidian: resources.obsidian - 1,
                            geodes: resources.geodes,
                        },
                        time: state.time + 1,
                    })
                }

                if resources.can_afford_robot(blueprint, Robot::Geode) {
                    queue.push_back(State {
                        ore_robots: state.ore_robots,
                        clay_robots: state.clay_robots,
                        obsidian_robots: state.obsidian_robots,
                        geode_robots: state.geode_robots + 1,
                        resources_in_inventory: Resources {
                            ore: resources.ore - blueprint.geode_robot_ore,
                            clay: resources.clay,
                            obsidian: resources.obsidian - blueprint.geode_robot_obsidian,
                            geodes: resources.geodes - 1,
                        },
                        time: state.time + 1,
                    })
                }
            }
            // }

            println!("{} {}", blueprint.idx, maximum_geodes);

            maximum_geodes
        })
        .product();

    dbg!(input);
}
