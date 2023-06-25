use crate::monkey::Monkey;

const N_ROUNDS: usize = 10_000;

pub fn main(contents: String) -> Result<String, String> {
    let monkey_texts = contents.split("\n\n");
    let mut monkeys = monkey_texts
        .map(|m| Monkey::build_from_text(m))
        .collect::<Result<Vec<Monkey>, String>>()?;
    for round in 1..N_ROUNDS {
        if round % 1 == 0 {
            println!("Round {}", round);
        }
        for i in 0..monkeys.len() {
            let current_monkey = &mut monkeys[i];
            let items = current_monkey.take_turn(false);
            // Assign these items to the monkeys they belong to.
            for (item, next_monkey) in items {
                monkeys[next_monkey].add_item(item);
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| m.n_inspected)
        .collect::<Vec<usize>>();
    // Sort them by inspection count.
    inspection_counts.sort();
    inspection_counts.reverse();
    let (m1, m2) = (inspection_counts[0], inspection_counts[1]);

    Ok((m1 * m2).to_string())
}