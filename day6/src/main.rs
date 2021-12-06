use std::collections::HashMap;

fn main() {
    let mut aggregate_counters: HashMap<u64, u64> = HashMap::new();

    include_str!("input.txt")
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .for_each(|number| {
            *aggregate_counters.entry(number).or_default() += 1;
        });

    for _ in 1..=80 {
        aggregate_counters = advance_counters(aggregate_counters);
    }

    let part_1 = aggregate_counters.iter().fold(0, |acc, (_, &n)| acc + n);
    dbg!(part_1);

    for _ in 81..=256 {
        aggregate_counters = advance_counters(aggregate_counters);
    }

    let part_2 = aggregate_counters.iter().fold(0, |acc, (_, &n)| acc + n);
    dbg!(part_2);
}

fn advance_counters(counters: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_counters: HashMap<u64, u64> = HashMap::new();

    for (counter, amount) in counters {
        if counter != 0 {
            *new_counters.entry(counter - 1).or_default() += amount;
        } else {
            *new_counters.entry(6).or_default() += amount;
            new_counters.insert(8, amount);
        }
    }

    new_counters
}
