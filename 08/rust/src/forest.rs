use std::fmt;

// Trees have a height and nothing else.
pub type Tree = u32;

pub struct Forest {
    trees: Vec<Vec<Tree>>,
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
}