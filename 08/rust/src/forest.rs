use std::fmt;
use std::collections::HashSet;

// Trees have a height and nothing else.
pub type Tree = u32;

pub struct Forest {
    trees: Vec<Vec<Tree>>,
}

fn take_until_inclusive<T, F>(xs: &Vec<T>, f: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    let mut ys = vec![];
    for x in xs {
        ys.push(x);
        if f(x) {
            break;
        }
    }
    ys
}

impl Forest {
    pub fn new_from_text(s: String) -> Result<Self, String> {
        let trees = s
        .lines()
        .map(|line| {
            line
            .chars()
            .map(|c| match c.to_digit(10) {
                Some(u) => Ok(u),
                None => Err(format!("invalid character: {}", c)),
            })
            .collect::<Result<Vec<Tree>, String>>()
        }).collect::<Result<Vec<Vec<Tree>>, String>>()?;

        Ok(Forest { trees })
    }

    pub fn at(&self, x: usize, y: usize) -> Result<&Tree, String> {
        self.trees
            .get(y)
            .and_then(|row| row.get(x))
            .ok_or_else(|| format!("out of bounds at {}, {}", x, y))
    }

    pub fn left_of(&self, x: usize, y: usize) -> Result<Vec<Tree>, String> {
        // Get all the trees to the right of (x, y).
        let mut trees = vec![];
        for x in (0..x).rev() {
            trees.push(*self.at(x, y)?);
        } 
        Ok(trees)
    }

    pub fn right_of(&self, x: usize, y: usize) -> Result<Vec<Tree>, String> {
        // Get all the trees to the right of (x, y).
        let (cols, _) = self.dimensions();
        let mut trees = vec![];
        for x in (x + 1)..cols {
            trees.push(*self.at(x, y)?);
        } 
        Ok(trees)
    }

    pub fn above(&self, x: usize, y: usize) -> Result<Vec<Tree>, String> {
        // Get all the trees above (x, y).
        let mut trees = vec![];
        for y in (0..y).rev() {
            trees.push(*self.at(x, y)?);
        } 
        Ok(trees)
    }

    pub fn below(&self, x: usize, y: usize) -> Result<Vec<Tree>, String> {
        // Get all the trees below (x, y).
        let (_, rows) = self.dimensions();
        let mut trees = vec![];
        for y in (y + 1)..rows {
            trees.push(*self.at(x, y)?);
        } 
        Ok(trees)
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> Result<usize, String> {
        let current_tree = self.at(x, y)?.clone();

        let trees_left = self.left_of(x, y)?;
        let visible_left = take_until_inclusive(&trees_left, |t| t >= &current_tree);
        let left_score = visible_left.len();

        let trees_right = self.right_of(x, y)?;
        let visible_right = take_until_inclusive(&trees_right, |t| t >= &current_tree);
        let right_score = visible_right.len();

        let trees_above = self.above(x, y)?;
        let visible_above = take_until_inclusive(&trees_above, |t| t >= &current_tree);
        let above_score = visible_above.len();

        let trees_below = self.below(x, y)?;
        let visible_below = take_until_inclusive(&trees_below, |t| t >= &current_tree);
        let below_score = visible_below.len();

        Ok(left_score * right_score * above_score * below_score)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let width = self.trees.len();
        let height = self.trees[0].len();
        (width, height)
    }

    fn visible_from_top(&self) -> Vec<(usize, usize)> {
        let (cols, rows) = self.dimensions();

        let mut visible = vec![];
        for x in 0..cols {
            // Track the tallest tree we've seen so we know if trees behind it are visible.
            let mut tallest_seen: i32 = -1;
            for y in 0..rows {
                let tree = self.at(x, y).unwrap();
                if (*tree as i32) > tallest_seen {
                    visible.push((x,y));
                    tallest_seen = tree.clone() as i32;
                }
            }
        }
        visible
    }

    fn visible_from_bottom(&self) -> Vec<(usize, usize)> {
        let (cols, rows) = self.dimensions();

        let mut visible = vec![];
        for x in 0..cols {
            // Track the tallest tree we've seen so we know if trees behind it are visible.
            let mut tallest_seen: i32 = -1;
            for from_bottom in 0..rows {
                let y = rows - (from_bottom + 1);
                let tree = self.at(x, y).unwrap();
                if (*tree as i32) > tallest_seen {
                    visible.push((x,y));
                    tallest_seen = tree.clone() as i32;
                }
            }
        }
        visible
    }

    fn visible_from_left(&self) -> Vec<(usize, usize)> {
        let (cols, rows) = self.dimensions();

        let mut visible = vec![];
        for y in 0..rows {
            let mut tallest_seen: i32 = -1;
            for x in 0..cols {
                let tree = self.at(x, y).unwrap();
                if (*tree as i32) > tallest_seen {
                    visible.push((x,y));
                    tallest_seen = tree.clone() as i32;
                }
            }
        }
        visible
    }

    fn visible_from_right(&self) -> Vec<(usize, usize)> {
        let (cols, rows) = self.dimensions();

        let mut visible = vec![];
        for y in 0..rows {
            let mut tallest_seen: i32 = -1;
            for from_right in 0..cols {
                let x = cols - (from_right + 1);
                let tree = self.at(x, y).unwrap();
                if (*tree as i32) > tallest_seen {
                    visible.push((x,y));
                    tallest_seen = tree.clone() as i32;
                }
            }
        }
        visible
    }

    pub fn visible_positions(&self) -> Vec<(usize, usize)> {
        // Find all tree positions that are visible from outside the grid.

        let visible_sets = vec![
            self.visible_from_top(),
            self.visible_from_bottom(),
            self.visible_from_left(),
            self.visible_from_right(),
        ];

        let mut visible = HashSet::new();
        visible_sets.iter().for_each(|visible_set| {
            visible.extend(visible_set);
        });

        visible.into_iter().collect()
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.trees {
            for tree in row {
                write!(f, "{}", tree)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_text() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.trees.len(), 3);
        assert_eq!(forest.trees[0], vec![1, 2, 3]);
        assert_eq!(forest.trees[1], vec![4, 5, 6]);
        assert_eq!(forest.trees[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_at() {
        let s = String::from("1234\n2345\n3456\n4567");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.at(0, 0).unwrap(), &1);
        assert_eq!(forest.at(1, 1).unwrap(), &3);
        assert_eq!(forest.at(3, 0).unwrap(), &4);
        assert_eq!(forest.at(0, 2).unwrap(), &3);
    }

    #[test]
    fn test_display() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(format!("{}", forest), "123\n456\n789\n");
    }

    #[test]
    fn test_right_of() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.right_of(0, 0).unwrap(), vec![2, 3]);
        assert_eq!(forest.right_of(1, 0).unwrap(), vec![3]);
        assert_eq!(forest.right_of(2, 0).unwrap(), vec![]);
        assert_eq!(forest.right_of(1, 1).unwrap(), vec![6]);
    }

    #[test]
    fn test_left_of() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.left_of(0, 0).unwrap(), vec![]);
        assert_eq!(forest.left_of(1, 0).unwrap(), vec![1]);
        assert_eq!(forest.left_of(2, 0).unwrap(), vec![2, 1]);
        assert_eq!(forest.left_of(1, 1).unwrap(), vec![4]);
    }

    #[test]
    fn test_above() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.above(0, 0).unwrap(), vec![]);
        assert_eq!(forest.above(1, 0).unwrap(), vec![]);
        assert_eq!(forest.above(0, 1).unwrap(), vec![1]);
        assert_eq!(forest.above(2, 2).unwrap(), vec![6, 3]);
    }

    #[test]
    fn test_below() {
        let s = String::from("123\n456\n789");
        let forest = Forest::new_from_text(s).unwrap();
        assert_eq!(forest.below(0, 0).unwrap(), vec![4, 7]);
        assert_eq!(forest.below(1, 0).unwrap(), vec![5, 8]);
        assert_eq!(forest.below(0, 1).unwrap(), vec![7]);
        assert_eq!(forest.below(2, 2).unwrap(), vec![]);
    }

    #[test]
    fn test_scenic_score() {
        let s = String::from("30373\n25512\n65332\n33549\n35390");
        let forest = Forest::new_from_text(s).unwrap();
        
        // Trees on the edge will have a score of 0 in at least one direction.
        assert_eq!(forest.scenic_score(0, 0), Ok(0));
        assert_eq!(forest.scenic_score(1, 0), Ok(0));
        assert_eq!(forest.scenic_score(0, 1), Ok(0));
        assert_eq!(forest.scenic_score(4, 2), Ok(0));
        assert_eq!(forest.scenic_score(2, 4), Ok(0));

        assert_eq!(forest.at(2, 3), Ok(&5));
        assert_eq!(forest.scenic_score(2, 3), Ok(8));
    }
}