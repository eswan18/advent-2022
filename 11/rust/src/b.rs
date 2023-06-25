use crate::monkey::Monkey;

const N_ROUNDS: usize = 10_000;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(numbers: Vec<i32>) -> i32 {
    let mut lcm = numbers[0];
    for i in 1..numbers.len() {
        lcm = (lcm * numbers[i]) / gcd(lcm, numbers[i]);
    }
    lcm
}

pub fn main(contents: String) -> Result<String, String> {
    let monkey_texts = contents.split("\n\n");
    let mut monkeys = monkey_texts
        .map(|m| Monkey::build_from_text(m))
        .collect::<Result<Vec<Monkey>, String>>()?;
    let divisors = monkeys.iter().map(|m| m.divisor).collect::<Vec<i32>>();
    let lcm = lcm(divisors);
    monkeys.iter_mut().for_each(|m| m.set_lcm(lcm));
    for _round in 1..(N_ROUNDS + 1) {
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
    println!("Inspection counts: {:?}", inspection_counts);
    let (m1, m2) = (inspection_counts[0], inspection_counts[1]);

    Ok((m1 * m2).to_string())
}
