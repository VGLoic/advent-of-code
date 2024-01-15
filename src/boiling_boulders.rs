use std;

pub fn derive_surface_area(
    filename: &str,
    filter_inner_air_pockets: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut lava_structure = LavaStructure::new();

    for line in content.lines() {
        let p = Position::try_from(line)?;
        lava_structure.add_droplet(p);
    }

    if !filter_inner_air_pockets {
        return Ok(lava_structure.max_surface_area);
    }

    Ok(lava_structure.max_surface_area - lava_structure.derive_inner_air_pockets_surface_area())
}

struct LavaStructure {
    max_surface_area: usize,
    droplet_positions: std::collections::HashSet<Position>,
    potential_water_positions: std::collections::HashSet<Position>,
    highest_coordinate: isize,
}

impl LavaStructure {
    fn new() -> Self {
        let droplet_positions = std::collections::HashSet::new();
        let potential_water_positions = std::collections::HashSet::new();
        LavaStructure {
            max_surface_area: 0,
            droplet_positions,
            potential_water_positions,
            highest_coordinate: 0,
        }
    }

    fn add_droplet(&mut self, p: Position) {
        let new_water_positions = self.potential_water_positions(&p);
        let surfaces_in_contact = 6 - new_water_positions.len();
        self.max_surface_area += 6;
        self.max_surface_area -= 2 * surfaces_in_contact;

        if p.x > self.highest_coordinate {
            self.highest_coordinate = p.x;
        }
        if p.y > self.highest_coordinate {
            self.highest_coordinate = p.y;
        }
        if p.z > self.highest_coordinate {
            self.highest_coordinate = p.z;
        }

        self.potential_water_positions.remove(&p);
        self.droplet_positions.insert(p);
        for water_p in new_water_positions {
            self.potential_water_positions.insert(water_p);
        }
    }

    fn potential_water_positions(&self, p: &Position) -> Vec<Position> {
        p.around_positions()
            .into_iter()
            .filter(|p| !self.droplet_positions.contains(p))
            .collect()
    }

    fn derive_inner_air_pockets_surface_area(&self) -> usize {
        let mut inner_surface_area = 0;
        let mut valid_water_positions = std::collections::HashSet::new();
        let mut invalid_water_positions = std::collections::HashSet::new();

        for p in &self.potential_water_positions {
            if valid_water_positions.contains(p) {
                continue;
            }

            if invalid_water_positions.contains(p) {
                inner_surface_area += p
                    .around_positions()
                    .into_iter()
                    .filter(|p| self.droplet_positions.contains(&p))
                    .count();
                continue;
            }

            let mut visited_water_positions: std::collections::HashSet<Position> =
                std::collections::HashSet::new();
            // If an around position is neither water, neither rock and we don't find rock at infinity, we have a real water position
            if self.has_air_nearby(&mut visited_water_positions, p) {
                for visited_p in visited_water_positions {
                    valid_water_positions.insert(visited_p);
                }
            } else {
                inner_surface_area += p
                    .around_positions()
                    .into_iter()
                    .filter(|p| self.droplet_positions.contains(&p))
                    .count();
                for visited_p in visited_water_positions {
                    invalid_water_positions.insert(visited_p);
                }
            }
        }

        inner_surface_area
    }

    fn has_air_nearby(
        &self,
        visited_water_positions: &mut std::collections::HashSet<Position>,
        p: &Position,
    ) -> bool {
        visited_water_positions.insert(p.clone());

        // If an around position is neither water, neither rock and we don't find rock at infinity, we have a real water position
        if p.around_positions().iter().any(|around_p| {
            !self.potential_water_positions.contains(&around_p)
                && !self.droplet_positions.contains(&around_p)
                && self.has_no_rock_to_infinity(p, around_p)
        }) {
            // println!("All air, all good");
            return true;
        }

        // Else, the position will ask its around water positions to tell if it has some air nearby

        let unvisited_water_positions = p
            .around_positions()
            .into_iter()
            .filter(|other_p| {
                self.potential_water_positions.contains(other_p)
                    && !visited_water_positions.contains(other_p)
            })
            .collect::<Vec<Position>>();

        for around_p in &unvisited_water_positions {
            // println!("Digging a bit {d_index}");
            if self.has_air_nearby(visited_water_positions, around_p) {
                // println!("Found air by digging a bit: {around_p}");
                return true;
            }
        }

        return false;
    }

    fn has_no_rock_to_infinity(&self, p0: &Position, p1: &Position) -> bool {
        let mut i = 0;
        let v = p1.sub(p0);
        let mut p = p0.add(&v);
        while i < self.highest_coordinate {
            p = p.add(&v);
            if self.droplet_positions.contains(&p) {
                return false;
            }
            i += 1
        }
        return true;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn around_positions(&self) -> Vec<Position> {
        vec![
            self.above(),
            self.under(),
            self.right_side(),
            self.left_side(),
            self.in_front(),
            self.behind(),
        ]
    }

    fn above(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }
    fn under(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    fn right_side(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }

    fn left_side(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    fn in_front(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }

    fn behind(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    fn add(&self, p: &Position) -> Position {
        Position {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }

    fn sub(&self, p: &Position) -> Position {
        Position {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }
}

impl TryFrom<&str> for Position {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coordinates = value
            .split(",")
            .map(|s| s.parse::<isize>().map_err(|e| e.into()))
            .collect::<Result<Vec<isize>, Box<dyn std::error::Error>>>()?;
        if coordinates.len() != 3 {
            return Err(format!("Invalid number of coordinates, expected 3, got {}. Value must be of the form `<x coordinate>,<y coordinate>,<z coordinate>`, got {}", coordinates.len(), value).into());
        }

        Ok(Position {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            derive_surface_area("inputs/input-18-example.txt", false).unwrap(),
            64
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            derive_surface_area("inputs/input-18.txt", false).unwrap(),
            4604
        );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            derive_surface_area("inputs/input-18-example.txt", true).unwrap(),
            58
        );
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            derive_surface_area("inputs/input-18.txt", true).unwrap(),
            2604
        );
    }
}
