use std::{num::ParseIntError, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Item(u128);

type ItemUpdater = dyn Fn(Item) -> Item;

pub struct Monkey {
    pub id: usize,
    items: Vec<Item>,
    updater: Box<ItemUpdater>,
    pub divisor: i32,
    // Which monkeys to throw items to on true/false test result.
    on_true: usize,
    on_false: usize,
    pub n_inspected: usize,
    lcm: Option<i32>,
}

impl Monkey {
    #[allow(dead_code)]
    pub fn new(
        id: usize,
        updater: Box<ItemUpdater>,
        divisor: i32,
        on_true: usize,
        on_false: usize,
        n_inspected: usize,
        lcm: Option<i32>,
    ) -> Monkey {
        Monkey {
            id,
            items: vec![],
            updater,
            divisor,
            on_true,
            on_false,
            n_inspected,
            lcm,
        }
    }

    pub fn update_item(&self, i: Item) -> Item {
        let update = &self.updater;
        update(i)
    }

    pub fn test_item(&self, i: &Item) -> bool {
        (i.0 as i32 % self.divisor) == 0
    }

    pub fn add_item(&mut self, i: Item) {
        self.items.push(i);
    }

    pub fn pop_item(&mut self, reduce: bool) -> Option<(Item, usize)> {
        if self.items.len() == 0 {
            return None;
        }
        // Get the first item and adjust its worry value.
        let mut item = self.items.remove(0);
        item = self.update_item(item);
        // You get a bit less worried...
        if reduce {
            item = Item(item.0 / 3);
        }
        if let Some(lcm) = &self.lcm {
            item = Item(item.0 % *lcm as u128);
        }
        // Decide where to send the item next.
        let next_monkey = if self.test_item(&item) {
            self.on_true
        } else {
            self.on_false
        };
        self.n_inspected += 1;
        Some((item, next_monkey))
    }

    pub fn take_turn(&mut self, reduce: bool) -> Vec<(Item, usize)> {
        let mut items = vec![];
        while let Some((item, next_monkey)) = self.pop_item(reduce) {
            items.push((item, next_monkey));
        }
        items
    }

    pub fn build_from_text(text: &str) -> Result<Monkey, String> {
        let lines: Vec<&str> = text.lines().collect();
        // ID
        let id_with_colon = lines[0]
            .split_whitespace()
            .nth(1)
            .ok_or("Could not parse id")?;
        let id = id_with_colon
            .split(":")
            .nth(0)
            .ok_or("Could not parse id")?;
        let id = id.parse::<usize>().map_err(|_| "Could not parse id")?;
        // Starting Items
        let items = lines[1]
            .split(": ")
            .nth(1)
            .ok_or("Could not parse starting items")?;
        let items = items
            .split(", ")
            .map(|i| i.parse::<u128>())
            .collect::<Result<Vec<u128>, ParseIntError>>()
            .map_err(|_| "Could not parse starting items")?;
        let items = items.iter().map(|i| Item(*i)).collect();
        // Updater
        let expr = lines[2]
            .split("new = ")
            .nth(1)
            .ok_or("Could not parse updater")?;
        let tokens = expr.split_whitespace().collect::<Vec<&str>>();
        let updater: Box<ItemUpdater> = match tokens[..] {
            ["old", "+", "old"] => Box::new(|i: Item| Item(i.0 + i.0)),
            ["old", "*", "old"] => Box::new(|i: Item| Item(i.0 * i.0)),
            ["old", "*", x] | [x, "*", "old"] => {
                let x = x.parse::<u128>().map_err(|_| "Could not parse updater")?;
                Box::new(move |i: Item| Item(i.0 * x))
            }
            ["old", "+", x] | [x, "+", "old"] => {
                let x = x.parse::<u128>().map_err(|_| "Could not parse updater")?;
                Box::new(move |i: Item| Item(i.0 + x))
            }
            _ => return Err("Could not parse updater".to_string()),
        };
        // Divisor
        let divisor = lines[3]
            .split("Test: divisible by ")
            .nth(1)
            .ok_or("Could not parse divisor")?;
        let divisor = divisor
            .parse::<i32>()
            .map_err(|_| "Could not parse divisor")?;
        // On True
        let on_true = lines[4]
            .split("throw to monkey ")
            .nth(1)
            .ok_or("Could not parse on_true")?
            .parse::<usize>()
            .map_err(|_| "Could not parse on_true")?;
        // On False
        let on_false = lines[5]
            .split("throw to monkey ")
            .nth(1)
            .ok_or("Could not parse on_false")?
            .parse::<usize>()
            .map_err(|_| "Could not parse on_false")?;

        Ok(Monkey {
            id,
            items,
            updater,
            divisor,
            on_true,
            on_false,
            n_inspected: 0,
            lcm: None,
        })
    }

    pub fn set_lcm(&mut self, lcm: i32) {
        self.lcm = Some(lcm);
    }
}

// Implement a simple Display for the monkey struct
impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|i| i.0.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "Monkey {}:\n  Starting items: {}\n  Test: divisible by {}\n    If true: throw to monkey {}\n    If false: throw to monkey {}",
            self.id, items, self.divisor, self.on_true, self.on_false
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let m = Monkey::new(0, Box::new(|i| i), 2, 0, 0, 0, None);
        assert_eq!(m.test_item(&Item(0)), true);
        assert_eq!(m.test_item(&Item(1)), false);
        assert_eq!(m.test_item(&Item(2)), true);
        assert_eq!(m.test_item(&Item(3)), false);
    }

    #[test]
    fn test_update() {
        let m = Monkey::new(0, Box::new(|i| Item(i.0 * 2 + 1)), 1, 0, 0, 0, None);
        assert_eq!(m.update_item(Item(0)), Item(1));
        assert_eq!(m.update_item(Item(1)), Item(3));
        assert_eq!(m.update_item(Item(2)), Item(5));
        assert_eq!(m.update_item(Item(3)), Item(7));
    }

    #[test]
    fn test_build_from_text() {
        let text = "Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";
        let m = Monkey::build_from_text(text).unwrap();

        assert_eq!(m.id, 3);
        assert_eq!(m.items, vec![Item(74)]);
        assert_eq!(m.update_item(Item(0)), Item(3));
        assert_eq!(m.update_item(Item(3)), Item(6));
        assert_eq!(m.divisor, 17);
        assert_eq!(m.on_true, 0);
        assert_eq!(m.on_false, 1);
    }
}
