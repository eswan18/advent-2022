use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub struct Item(usize);

type ItemUpdater = dyn Fn(Item) -> Item;
type ItemTester = dyn Fn(Item) -> bool;

impl Item {
    fn new() -> Item {
        Item(0)
    }
}

pub struct Monkey {
    id: usize,
    items: Vec<Item>,
    updater: Box<ItemUpdater>,
    tester: Box<ItemTester>,
    // Which monkeys to throw items to on true/false test result.
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    pub fn new(
        id: usize,
        updater: Box<ItemUpdater>,
        tester: Box<ItemTester>,
        on_true: usize,
        on_false: usize,
    ) -> Monkey {
        Monkey {
            id,
            items: vec![],
            updater,
            tester,
            on_true,
            on_false,
        }
    }

    pub fn update_item(&self, i: Item) -> Item {
        let update = &self.updater;
        update(i)
    }

    pub fn test_item(&self, i: Item) -> bool {
        let test = &self.tester;
        test(i)
    }

    pub fn build_from_text(text: &str) -> Result<Monkey, String> {
        let lines:Vec<&str> = text.lines().collect();
        // ID
        let id_with_colon = lines[0].split_whitespace().nth(1).ok_or("Could not parse id")?;
        let id = id_with_colon.split(":").nth(0).ok_or("Could not parse id")?;
        let id = id.parse::<usize>().map_err(|_| "Could not parse id")?;
        // Starting Items
        let items = lines[1].split(": ").nth(1).ok_or("Could not parse starting items")?;
        let items = items
            .split(", ")
            .map(|i| i.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .map_err(|_| "Could not parse starting items")?;
        let items = items
            .iter()
            .map(|i| Item(*i))
            .collect();
        // Updater
        let expr = lines[2].split("new = ").nth(1).ok_or("Could not parse updater")?;
        let tokens = expr.split_whitespace().collect::<Vec<&str>>();
        let updater: Box<ItemUpdater> = match tokens[..] {
            ["old", "+", "old"] => Box::new(|i: Item| Item(i.0 + i.0)),
            ["old", "*", "old"] => Box::new(|i: Item| Item(i.0 * i.0)),
            ["old", "*", x ] | [x, "*", "old"] => {
                let x = x.parse::<usize>().map_err(|_| "Could not parse updater")?;
                Box::new(move |i: Item| Item(i.0 * x))
            },
            ["old", "+", x ] | [x, "+", "old"] => {
                let x = x.parse::<usize>().map_err(|_| "Could not parse updater")?;
                Box::new(move |i: Item| Item(i.0 + x))
            },
            _ => return Err("Could not parse updater".to_string()),
        };

        Ok(Monkey {
            id,
            items,
            updater: updater,
            tester: Box::new(|_| false),
            on_true: 1,
            on_false: 2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let m = Monkey::new(0, Box::new(|i| i), Box::new(|i| i.0 % 2 == 0), 0, 0);
        assert_eq!(m.test_item(Item(0)), true);
        assert_eq!(m.test_item(Item(1)), false);
        assert_eq!(m.test_item(Item(2)), true);
        assert_eq!(m.test_item(Item(3)), false);
    }

    #[test]
    fn test_update() {
        let m = Monkey::new(0, Box::new(|i| Item(i.0 * 2 + 1)), Box::new(|_| true), 0, 0);
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
    }
}
