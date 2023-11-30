use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
};

#[allow(dead_code)]
pub fn count_visible_trees(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    let forest = Forest::try_from_raw_grid(content)?;

    println!("Forest: {}", forest);

    Ok(forest.inner_visible_trees().len() + 4 * (forest.dimension() - 1))
}

pub fn find_highest_scenic_score(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    let forest = Forest::try_from_raw_grid(content)?;

    println!("Forest: {}", forest);

    Ok(forest.inner_visible_trees_highest_scenic_score())
}

#[derive(Debug)]
struct ScenicScore {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<u8>>,
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let displayed = &self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        return write!(f, "\n{}\n", displayed);
    }
}

impl Forest {
    fn inner_visible_trees_highest_scenic_score(&self) -> usize {
        let dimension = self.dimension();

        let mut highest_trees_from_top = self.grid[0].clone();
        let mut highest_trees_from_bottom = self.grid[dimension - 1].clone();

        let mut visible_tree_scenic_scores: HashMap<(usize, usize), ScenicScore> = HashMap::new();

        for i in 1..dimension - 1 {
            let mut highest_tree_from_left = self.grid[i][0];

            let mut highest_tree_from_right = self.grid[dimension - i - 1][dimension - 1];

            for j in 1..dimension - 1 {
                let tree = self.grid[i][j];

                let is_visible_from_left = tree > highest_tree_from_left;
                let is_visible_from_top = tree > highest_trees_from_top[j];
                let has_already_been_detected_as_visible_from_right_or_bottom =
                    visible_tree_scenic_scores.contains_key(&(i, j));

                if is_visible_from_left
                    || is_visible_from_top
                    || has_already_been_detected_as_visible_from_right_or_bottom
                {
                    let left_score = if is_visible_from_left { j } else { 0 };
                    let top_score = if is_visible_from_top { i } else { 0 };

                    let visible_tree_scenic_score = visible_tree_scenic_scores.get_mut(&(i, j));
                    if let Some(score) = visible_tree_scenic_score {
                        score.left = left_score;
                        score.top = top_score;
                    } else {
                        visible_tree_scenic_scores.insert(
                            (i, j),
                            ScenicScore {
                                top: top_score,
                                right: 0,
                                bottom: 0,
                                left: left_score,
                            },
                        );
                    }
                }

                if is_visible_from_left {
                    highest_tree_from_left = tree;
                }
                if is_visible_from_top {
                    highest_trees_from_top[j] = tree;
                }

                let inverse_tree = self.grid[dimension - 1 - i][dimension - 1 - j];

                let is_visible_from_right = inverse_tree > highest_tree_from_right;
                let is_visible_from_bottom =
                    inverse_tree > highest_trees_from_bottom[dimension - 1 - j];
                let has_already_been_detected_as_visible_from_left_or_top =
                    visible_tree_scenic_scores
                        .contains_key(&(dimension - 1 - i, dimension - 1 - j));

                if is_visible_from_right
                    || is_visible_from_bottom
                    || has_already_been_detected_as_visible_from_left_or_top
                {
                    let right_score = if is_visible_from_right { j } else { 0 };
                    let bottom_score = if is_visible_from_bottom { i } else { 0 };
                    let visible_tree_scenic_score =
                        visible_tree_scenic_scores.get_mut(&(dimension - 1 - i, dimension - 1 - j));

                    if let Some(score) = visible_tree_scenic_score {
                        score.right = right_score;
                        score.bottom = bottom_score;
                    } else {
                        visible_tree_scenic_scores.insert(
                            (dimension - 1 - i, dimension - 1 - j),
                            ScenicScore {
                                top: 0,
                                right: right_score,
                                bottom: bottom_score,
                                left: 0,
                            },
                        );
                    }
                }

                if is_visible_from_right {
                    highest_tree_from_right = inverse_tree;
                }
                if is_visible_from_bottom {
                    highest_trees_from_bottom[dimension - 1 - j] = inverse_tree;
                }
            }
        }

        let mut max_score = 0;
        for ((row, column), scenic_score) in visible_tree_scenic_scores.iter_mut() {
            let visible_tree = self.grid[*row][*column];
            if scenic_score.top == 0 {
                let mut i = row - 1;
                loop {
                    let tree = self.grid[i][*column];
                    if tree >= visible_tree {
                        scenic_score.top = row - i;
                        break;
                    }
                    if i == 0 {
                        panic!("Oopsie, shoult not have reached the edge from the top");
                    }
                    i -= 1;
                }
            }
            if scenic_score.right == 0 {
                let mut j = column + 1;
                while j < dimension {
                    let tree = self.grid[*row][j];
                    if tree >= visible_tree {
                        scenic_score.right = j - column;
                        break;
                    }
                    j += 1;
                }
            }
            if scenic_score.bottom == 0 {
                let mut i = row + 1;
                while i < dimension {
                    let tree = self.grid[i][*column];
                    if tree >= visible_tree {
                        scenic_score.bottom = i - row;
                        break;
                    }
                    i += 1;
                }
            }
            if scenic_score.left == 0 {
                let mut j = column - 1;
                loop {
                    let tree = self.grid[*row][j];
                    if tree >= visible_tree {
                        scenic_score.left = column - j;
                        break;
                    }
                    if j == 0 {
                        panic!("Oopsie, shoult not have reached the edge from the left");
                    }
                    j -= 1;
                }
            }
            let score =
                scenic_score.top * scenic_score.right * scenic_score.bottom * scenic_score.left;
            if score > max_score {
                max_score = score;
            }
        }

        max_score
    }

    fn inner_visible_trees(&self) -> HashSet<(usize, usize)> {
        let dimension = self.dimension();

        let mut highest_trees_from_top = self.grid[0].clone();
        let mut highest_trees_from_bottom = self.grid[dimension - 1].clone();

        let mut visible_trees = HashSet::new();

        for i in 1..dimension - 1 {
            let mut highest_tree_from_left = self.grid[i][0];

            let mut highest_tree_from_right = self.grid[dimension - i - 1][dimension - 1];

            for j in 1..dimension - 1 {
                let tree = self.grid[i][j];

                let is_visible_from_left = tree > highest_tree_from_left;
                let is_visible_from_top = tree > highest_trees_from_top[j];

                if is_visible_from_left || is_visible_from_top {
                    visible_trees.insert((i, j));
                }

                if is_visible_from_left {
                    highest_tree_from_left = tree;
                }
                if is_visible_from_top {
                    highest_trees_from_top[j] = tree;
                }

                let inverse_tree = self.grid[dimension - 1 - i][dimension - 1 - j];

                let is_visible_from_right = inverse_tree > highest_tree_from_right;
                let is_visible_from_bottom =
                    inverse_tree > highest_trees_from_bottom[dimension - 1 - j];

                if is_visible_from_right || is_visible_from_bottom {
                    visible_trees.insert((dimension - 1 - i, dimension - 1 - j));
                }

                if is_visible_from_right {
                    highest_tree_from_right = inverse_tree;
                }
                if is_visible_from_bottom {
                    highest_trees_from_bottom[dimension - 1 - j] = inverse_tree;
                }
            }
        }

        visible_trees
    }

    fn dimension(&self) -> usize {
        self.grid.len()
    }

    fn try_from_raw_grid(content: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut grid: Vec<Vec<u8>> = vec![];

        let first_line = content.lines().take(1).collect::<Vec<&str>>()[0];
        let first_row = line_to_row(first_line)?;

        let row_dimension = first_row.len();

        grid.push(first_row);

        for line in content.lines().skip(1) {
            let row = line_to_row(line)?;
            if row.len() != row_dimension {
                return Err(format!(
                    "Found line of different length than the first one, expected {}, got {}",
                    row_dimension,
                    row.len()
                )
                .into());
            }
            grid.push(row);
        }

        if grid.len() != row_dimension {
            return Err(format!("Dimensions of the grid are invalid, expected a square, got a rectangle with {} rows and {} columns", row_dimension, grid.len()).into());
        }

        Ok(Forest { grid })
    }
}

fn line_to_row(line: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    line.chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(format!("Unable to parse char {} into an integer", c))
                .and_then(|x| {
                    let y: Result<u8, _> = x.try_into().map_err(|_| {
                        format!("Unable to convert parsed value {} from u32 to u8", x)
                    });
                    y
                })
        })
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(
            count_visible_trees("inputs/input-08-example.txt").unwrap(),
            21
        );
    }

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(count_visible_trees("inputs/input-08.txt").unwrap(), 1816);
    }

    #[test]
    fn example_part_2_has_right_answer() {
        assert_eq!(
            find_highest_scenic_score("inputs/input-08-example.txt").unwrap(),
            8
        );
    }

    #[test]
    fn part_2_has_right_answer() {
        assert_eq!(
            find_highest_scenic_score("inputs/input-08.txt").unwrap(),
            383520
        );
    }
}
