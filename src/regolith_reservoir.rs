use std;

pub fn find_number_of_resting_units_of_sand(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    let mut rock_set = vec![];
    for line in content.lines() {
        let mut points = vec![];
        for raw_point in line.split("->") {
            let point = Point::try_from(raw_point.trim().trim_end())?;
            if point.x > max_x {
                max_x = point.x;
            }
            if point.x < min_x {
                min_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
            println!("Point: {point}");
            points.push(point);
        }
        rock_set.push(points);
    }

    let mut grid = Cave::build_empty_cave(min_x, max_x - min_x + 1, 0, max_y + 1)?;

    for i in 0..rock_set.len() {
        for j in 1..rock_set[i].len() {
            grid.draw_rock_line(&rock_set[i][j - 1], &rock_set[i][j])?;
        }
    }

    println!("Grid {grid}");
    println!("Minimum x {min_x}");
    println!("Maximum x {max_x}");

    Ok(3)
}

#[derive(Debug, Clone)]
enum CaveElement {
    Air,
    Rock,
    Sand
}
struct Cave {
    x_offset: usize,
    y_offset: usize,
    grid: Vec<Vec<CaveElement>>
}

impl Cave {
    fn build_empty_cave(from_x: usize, x_dimension: usize, from_y: usize, y_dimension: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let x_offset = from_x;
        let y_offset = from_y;

        if x_dimension == 0 {
            return Err("Dimension along X axis can not be 0".into());
        }

        if x_offset + x_dimension >= 1_000 {
            return Err("Offset or dimension along the X axis is too large, only point until 1000 are supported".into());
        }

        if y_dimension == 0 {
            return Err("Dimension along Y axis can not be 0".into());
        }

        if y_offset + y_dimension >= 1_000 {
            return Err("Offset or dimension along the Y axis is too large, only point until 1000 are supported".into());
        }

        let empty_row = vec![CaveElement::Air; x_dimension];
        let empty_grid = vec![empty_row; y_dimension];

        Ok(Cave { grid: empty_grid, x_offset, y_offset })
    }

    fn x_dimension(&self) -> usize {
        self.grid[0].len()
    }

    fn x_starting_point(&self) -> usize {
        self.x_offset
    }

    fn y_starting_point(&self) -> usize {
        self.y_offset
    }

    fn y_dimension(&self) -> usize {
        self.grid.len()
    }

    fn draw_rock_line(&mut self, a: &Point, b: &Point) -> Result<(), Box<dyn std::error::Error>> {
        if a.x != b.x {
            if a.y != b.y {
                return Err(format!("Drawing diagonal line is not supported, only vertical and horizontal. Given points are invalid in that regard. Points {} and {}.", a, b).into());
            }
            // Draw along X
            let (start, end) = if a.x > b.x {
                (b.x, a.x)
            } else {
                (a.x, b.x)
            };
            for i in start..end {
                self.grid[a.y - self.y_offset][i - self.x_offset] = CaveElement::Rock;
            }
            self.grid[a.y - self.y_offset][end - self.x_offset] = CaveElement::Rock;
        } else {
            // Draw along Y
            let (start, end) = if a.y > b.y {
                 (b.y, a.y)
             } else {
                 (a.y, b.y)
             };
             for j in start..end {
                 self.grid[j - self.y_offset][a.x - self.x_offset] = CaveElement::Rock;
             }
             self.grid[end - self.y_offset][a.x - self.x_offset] = CaveElement::Rock;
        }

        Ok(())
    }
}

struct Point {
    x: usize,
    y: usize
}

impl TryFrom<&str> for Point {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coordinates = value.split(",")
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, std::num::ParseIntError>>()?;
        if coordinates.len() != 2 {
            return Err(format!("Need two coordinates to form a point, got {}. Original parsed string was {}", coordinates.len(), value).into());
        }
        Ok(Point { x: coordinates[0], y: coordinates[1] })
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut displayed = "".to_owned();
        let mut first_rows = vec!["    ".to_owned(); 3];
        
        for i in self.x_starting_point()..self.x_starting_point() + self.x_dimension() {
            let q_100 = i / 100;
            let q_10 = (i - q_100 * 100) / 10;
            let r = i % 10;
            first_rows[0] += q_100.to_string().as_str();
            first_rows[0] += " ";
            first_rows[1] += q_10.to_string().as_str();
            first_rows[1] += " ";
            first_rows[2] += r.to_string().as_str();
            first_rows[2] += " ";
        }
        displayed += first_rows.join("\n").as_str();
        displayed += "\n";
        for j in self.y_starting_point()..self.y_starting_point() + self.y_dimension() {
            let q_100 = j / 100;
            let q_10 = (j - q_100 * 100) / 10;
            let r = j % 10;
            displayed += q_100.to_string().as_str();
            displayed += q_10.to_string().as_str();
            displayed += r.to_string().as_str();
            displayed += " ";
            for i in 0..self.x_dimension() {
                let el = match self.grid[j - self.y_offset][i] {
                    CaveElement::Air => '.',
                    CaveElement::Sand => 'o',
                    CaveElement::Rock => '#'
                };
                displayed.push(el);
                displayed.push(' ');
            }
            displayed += "\n";
        }
        write!(f, "\n{}", displayed)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_answer() {
        assert_eq!(
            find_number_of_resting_units_of_sand("inputs/input-14-example.txt").unwrap(),
            24
        );
    }
}
