use std;

pub fn derive_surface_area(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut lava_structure = LavaStructure::new();

    for line in content.lines() {
        let p = Position::try_from(line)?;
        println!("{p}");
        lava_structure.add_droplet(p);
    }

    Ok(lava_structure.surface_area)
}

struct LavaStructure {
    surface_area: usize,
    droplet_positions: std::collections::HashSet<Position>
}

impl LavaStructure {
    fn new() -> Self {
        let droplet_positions = std::collections::HashSet::new();
        LavaStructure {
            surface_area: 0,
            droplet_positions
        }
    }

    fn add_droplet(&mut self, p: Position) {
        let surfaces_in_contact = self.count_surfaces_in_contact(&p);
        self.surface_area += 6;
        self.surface_area -= 2 * surfaces_in_contact;
        self.droplet_positions.insert(p);
    }

    fn count_surfaces_in_contact(&self, p: &Position) -> usize {
        let mut count = 0;
        // Above
        if self.droplet_positions.contains(&p.above()) {
            count += 1;
        }
        // Under
        if self.droplet_positions.contains(&p.under()) {
            count += 1;
        }
        // Right
        if self.droplet_positions.contains(&p.right_side()) {
            count += 1;
        }
        // Left
        if self.droplet_positions.contains(&p.left_side()) {
            count += 1;
        }
        // In front
        if self.droplet_positions.contains(&p.in_front()) {
            count += 1;
        }
        // Behind
        if self.droplet_positions.contains(&p.behind()) {
            count += 1;
        }
        count
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
