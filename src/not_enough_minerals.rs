use std;
use regex::Regex;

pub fn stuff(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut blueprints = vec![];

    for line in content.lines() {
        blueprints.push(Blueprint::try_from(line)?);
    }

    println!("Blueprints: {:?}", blueprints);

    Ok(3)
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: ObsidianRobot,
    geode_robot: GeodeRobot
}
#[derive(Debug)]
struct ObsidianRobot {
    ore: usize,
    clay: usize
}
#[derive(Debug)]
struct GeodeRobot {
    ore: usize,
    obsidian: usize
}


impl TryFrom<&str> for Blueprint {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(
            r"Blueprint (-?\d+): Each ore robot costs (-?\d+) ore. Each clay robot costs (-?\d+) ore. Each obsidian robot costs (-?\d+) ore and (-?\d+) clay. Each geode robot costs (-?\d+) ore and (-?\d+) obsidian.",
        )?;
        let captures = pattern.captures(value).ok_or(format!("Unable to parse line for Blueprint. Got `{}`", value))?;

        if captures.len() < 8 {
            return Err(format!("Unable to parse line into blueprint. Got {}", value).into());
        }

        let ore_robot = captures[2].parse::<usize>()?;
        let clay_robot = captures[3].parse::<usize>()?;
        let obsidian_robot_ore = captures[4].parse::<usize>()?;
        let obsidian_robot_clay = captures[5].parse::<usize>()?;
        let geode_robot_ore = captures[6].parse::<usize>()?;
        let geode_robot_obsidian = captures[7].parse::<usize>()?;

        Ok(Blueprint {
            ore_robot,
            clay_robot,
            obsidian_robot: ObsidianRobot {
                ore: obsidian_robot_ore,
                clay: obsidian_robot_clay
            },
            geode_robot: GeodeRobot {
                ore: geode_robot_ore,
                obsidian: geode_robot_obsidian
            }
        })
    }
}
