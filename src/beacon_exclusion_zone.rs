use std::{self, collections::HashSet};

use regex::Regex;

pub fn find_number_of_covered_positions_in_row(
    filename: &str,
    target_y: isize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut sensors = vec![];
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut occupied_beacons_x_positions = HashSet::new();
    for line in content.lines() {
        let sensor = Sensor::try_from(line)?;
        let d: isize = sensor.closest_beacon_distance.try_into()?;
        let sensor_min_x = sensor.position.x - d;
        let sensor_max_x = sensor.position.x + d;
        if sensor_min_x < min_x {
            min_x = sensor_min_x;
        }
        if sensor_max_x > max_x {
            max_x = sensor_max_x;
        }
        let beacon_x = sensor.closest_beacon_position.x;
        if sensor.closest_beacon_position.y == target_y {
            occupied_beacons_x_positions.insert(beacon_x);
        }
        sensors.push(sensor);
    }
    println!("Min x: {min_x}, max x: {max_x}");

    let mut covered_positions = 0;
    for x in min_x..max_x + 1 {
        if occupied_beacons_x_positions.contains(&x) {
            // println!("Found beacon at {x}");
            continue;
        }
        let p = Point::new(x, target_y);

        let mut is_position_covered = false;
        for sensor in &sensors {
            if sensor.within_distance(&p) {
                // let d = Point::distance(&sensor.position, &p);
                // println!("Covered:
                // x {x}
                // Distance to sensor: {d}
                // Distance covered by sensor: {}
                // ", sensor.closest_beacon_distance);
                is_position_covered = true;
                break;
            }
        }
        if is_position_covered {
            // println!("Covered: x {x}");
            covered_positions += 1;
        }
    }
    Ok(covered_positions)
}

pub fn find_distress_beacon_tuning_frequency(
    filename: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut sensors = vec![];
    let mut occupied_beacons_positions = HashSet::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for line in content.lines() {
        let sensor = Sensor::try_from(line)?;
        if sensor.position.x > max_x {
            max_x = sensor.position.x;
        }
        if sensor.position.x < min_x {
            min_x = sensor.position.x;
        }
        if sensor.position.y > max_y {
            max_y = sensor.position.y;
        }
        if sensor.position.x < min_y {
            min_y = sensor.position.y;
        }
        occupied_beacons_positions.insert(sensor.closest_beacon_position);
        sensors.push(sensor);
    }

    sensors.sort_unstable_by(|a, b| b.position.x.cmp(&a.position.x));

    min_x = std::cmp::max(0, min_x);
    max_x = std::cmp::min(4_000_000, max_x);
    min_y = std::cmp::max(0, min_y);
    max_y = std::cmp::min(4_000_000, max_y);

    println!(
        "Boundaries:
        X: {min_x} to {max_x}
        Y: {min_y} to {max_y}
    "
    );

    let mut x = min_x;
    let mut y = min_y;
    while y <= max_y {
        let p = Point::new(x, y);
        let mut is_position_covered = false;
        for sensor in &sensors {
            if sensor.within_distance(&p) {
                let available_x_distance =
                    sensor.closest_beacon_distance as isize - (p.y - sensor.position.y).abs();
                let next_x = sensor.position.x + available_x_distance;
                if next_x > max_x {
                    // println!("New line {}", y + 1);
                    x = min_x;
                    y += 1;
                } else if next_x == x {
                    // println!("At the tip {x}, increasing of 1");
                    x += 1;
                } else {
                    // println!("Jump from {x} to {next_x}");
                    x = next_x;
                }
                is_position_covered = true;
                break;
            }
        }
        if !is_position_covered {
            let tuning_frequency = p.x * 4_000_000 + p.y;
            println!(
                "Found uncovered position:
                X: {}
                Y: {}
                tuning frequency: {}
            ",
                p.x, p.y, tuning_frequency
            );
            return Ok(tuning_frequency as usize);
        }
    }
    Err("Unable to have found an uncovered position".into())
}

struct Sensor {
    position: Point,
    closest_beacon_position: Point,
    closest_beacon_distance: usize,
}

impl Sensor {
    fn within_distance(&self, p: &Point) -> bool {
        Point::distance(&self.position, p) <= self.closest_beacon_distance
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn distance(p0: &Point, p1: &Point) -> usize {
        return ((p0.x - p1.x).abs() + (p0.y - p1.y).abs()) as usize;
    }
}

impl TryFrom<&str> for Sensor {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )?;
        let captures = pattern.captures(value).ok_or(format!("Unable to parse line into sensor, expected line of format `Sensor at x=<value>, y=<value>: closest beacon is at x=<value>, y=<value>`. Got `{}`", value))?;
        if captures.len() < 5 {
            return Err(format!("Unable to parse line into sensor, expected line of format `Sensor at x=<value>, y=<value>: closest beacon is at x=<value>, y=<value>`. Got {}", value).into());
        }
        let sensor_x = captures[1].parse::<isize>().map_err(|e| {
            format!(
                "Invalid numerical value for the `x` position of sensor. Got err {}",
                e
            )
        })?;
        let sensor_y = captures[2].parse::<isize>().map_err(|e| {
            format!(
                "Invalid numerical value for the `y` position of sensor. Got err {}",
                e
            )
        })?;
        let beacon_x = captures[3].parse::<isize>().map_err(|e| {
            format!(
                "Invalid numerical value for the `x` position of beacon. Got err {}",
                e
            )
        })?;
        let beacon_y = captures[4].parse::<isize>().map_err(|e| {
            format!(
                "Invalid numerical value for the `y` position of beacon. Got err {}",
                e
            )
        })?;

        //         println!("New sensor:
        //         X: {sensor_x}
        //         Y: {sensor_y}
        // With beacon
        //         X: {beacon_x}
        //         Y: {beacon_y}
        //         ");

        let sensor_position = Point::new(sensor_x, sensor_y);
        let beacon_position = Point::new(beacon_x, beacon_y);

        let distance = Point::distance(&sensor_position, &beacon_position);

        Ok(Sensor {
            closest_beacon_distance: distance,
            closest_beacon_position: beacon_position,
            position: sensor_position,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            find_number_of_covered_positions_in_row("inputs/input-15-example.txt", 10).unwrap(),
            26
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            find_number_of_covered_positions_in_row("inputs/input-15.txt", 2_000_000).unwrap(),
            5716881
        );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            find_distress_beacon_tuning_frequency("inputs/input-15-example.txt").unwrap(),
            56000011
        );
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            find_distress_beacon_tuning_frequency("inputs/input-15.txt").unwrap(),
            10852583132904
        );
    }
}
