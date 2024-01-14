use std;

pub fn derive_surface_area(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut lava_structure = LavaStructure::new();

    for line in content.lines() {
        let p = Position::try_from(line)?;
        lava_structure.add_droplet(p);
    }

    Ok(lava_structure.max_surface_area)
}

struct LavaStructure {
    max_surface_area: usize,
    droplet_positions: std::collections::HashSet<Position>,
    potential_water_positions: std::collections::HashSet<Position>,
}

impl LavaStructure {
    fn new() -> Self {
        let droplet_positions = std::collections::HashSet::new();
        let potential_water_positions = std::collections::HashSet::new();
        LavaStructure {
            max_surface_area: 0,
            droplet_positions,
            potential_water_positions,
        }
    }

    fn add_droplet(&mut self, p: Position) {
        let new_water_positions = self.potential_water_positions(&p);
        let surfaces_in_contact = 6 - new_water_positions.len();
        self.max_surface_area += 6;
        self.max_surface_area -= 2 * surfaces_in_contact;
        self.droplet_positions.insert(p);
        for water_p in new_water_positions {
            self.potential_water_positions.insert(water_p);
        }
    }

    fn potential_water_positions(&self, p: &Position) -> Vec<Position> {
        let positions = vec![
            p.above(),
            p.under(),
            p.right_side(),
            p.left_side(),
            p.in_front(),
            p.behind()
        ];

        positions.into_iter()
            .filter(|p| !self.droplet_positions.contains(p))
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize
}

impl Position {
    fn above(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z + 1
        }
    }
    fn under(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z - 1
        }
    }

    fn right_side(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
            z: self.z
        }
    }

    fn left_side(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
            z: self.z
        }
    }

    fn in_front(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
            z: self.z
        }
    }

    fn behind(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
            z: self.z
        }
    }
}

impl TryFrom<&str> for Position {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coordinates = value.split(",")
            .map(|s| s.parse::<isize>().map_err(|e| e.into()))
            .collect::<Result<Vec<isize>, Box<dyn std::error::Error>>>()?;
        if coordinates.len() != 3 {
            return Err(format!("Invalid number of coordinates, expected 3, got {}. Value must be of the form `<x coordinate>,<y coordinate>,<z coordinate>`, got {}", coordinates.len(), value).into());
        }

        Ok(Position {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2]
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
            derive_surface_area("inputs/input-18-example.txt").unwrap(),
            64
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            derive_surface_area("inputs/input-18.txt").unwrap(),
            4604
        );
    }
}
