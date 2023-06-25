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
    items: Vec<Item>,
    updater: Box<ItemUpdater>,
    tester: Box<ItemTester>,
    // Which monkeys to throw items to on true/false test result.
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    pub fn new(
        updater: Box<ItemUpdater>,
        tester: Box<ItemTester>,
        on_true: usize,
        on_false: usize,
    ) -> Monkey {
        Monkey {
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let m = Monkey::new(
            Box::new(|i| i),
            Box::new(|i| i.0 % 2 == 0),
            0,
            0,
        );
        assert_eq!(m.test_item(Item(0)), true);
        assert_eq!(m.test_item(Item(1)), false);
        assert_eq!(m.test_item(Item(2)), true);
        assert_eq!(m.test_item(Item(3)), false);
    }

    #[test]
    fn test_update() {
        let m = Monkey::new(
            Box::new(|i| Item(i.0 * 2 + 1)),
            Box::new(|i| true ),
            0,
            0,
        );
        assert_eq!(m.update_item(Item(0)), Item(1));
        assert_eq!(m.update_item(Item(1)), Item(3));
        assert_eq!(m.update_item(Item(2)), Item(5));
        assert_eq!(m.update_item(Item(3)), Item(7));
    }
}