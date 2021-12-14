use std::collections::HashMap;

fn main() {
    let (mut polymer, transformations): (String, HashMap<(char, char), char>) =
        parse_input(include_str!("input.txt"));

    let mut new_polymer = polymer.clone();

    for i in 0..10 {
        new_polymer = step(new_polymer, &transformations);
    }

    let part_1 = analyse_polymer(&new_polymer);
    dbg!(part_1);

    let (parsed_polymer, first, last) = parse_polymer_to_pairs(&polymer);
    let mut new_polymer = parsed_polymer;

    for i in 0..40 {
        new_polymer = aggregate_step(new_polymer, &transformations);
    }

    let part_2 = analyse_agregate_polymer(count_letters(new_polymer, first, last));

    dbg!(part_2);
}

type Key = (char, char);

fn analyse_agregate_polymer(chars: HashMap<char, i64>) -> i64 {
    let mut chars: Vec<i64> = chars.values().map(|c| *c).collect();
    chars.sort();

    chars.last().unwrap() - chars.first().unwrap()
}

fn count_letters(polymer: HashMap<Key, i64>, first: Key, last: Key) -> HashMap<char, i64> {
    let mut count_map = HashMap::new();

    for ((a, b), count) in polymer {
        *count_map.entry(a).or_default() += count;
    }

    *count_map.entry(last.1).or_default() += 1;

    count_map
}

fn parse_polymer_to_pairs(polymer: &str) -> (HashMap<Key, i64>, Key, Key) {
    let mut parsed_polymer: HashMap<Key, i64> = HashMap::new();

    let chars = polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|key| (key[0], key[1]))
        .collect::<Vec<Key>>();

    for key in &chars {
        *parsed_polymer.entry(*key).or_default() += 1;
    }

    (
        parsed_polymer,
        *chars.first().unwrap(),
        *chars.last().unwrap(),
    )
}

fn step(polymer: String, transformations: &HashMap<Key, char>) -> String {
    let mut new_polymer = String::new();

    new_polymer.push(polymer.chars().nth(0).unwrap());

    let chars: Vec<char> = polymer.chars().collect();

    chars.windows(2).for_each(|(keys)| {
        let key = (keys[0], keys[1]);
        let new_char = transformations.get(&key).unwrap();
        new_polymer.push(*new_char);
        new_polymer.push(keys[1]);
    });
    new_polymer
}

fn aggregate_step(
    polymer: HashMap<Key, i64>,
    transformations: &HashMap<Key, char>,
) -> HashMap<Key, i64> {
    let mut new_polymer = HashMap::new();

    for ((a, b), i) in polymer {
        let middle = *transformations.get(&(a, b)).unwrap();

        *new_polymer.entry((a, middle)).or_default() += i;
        *new_polymer.entry((middle, b)).or_default() += i;
    }

    new_polymer
}

fn analyse_polymer(polymer: &String) -> i64 {
    let mut chars: HashMap<char, i64> = HashMap::new();

    polymer.chars().for_each(|c| {
        *chars.entry(c).or_default() += 1;
    });

    let mut chars: Vec<i64> = chars.values().map(|c| *c).collect();
    chars.sort();

    chars.last().unwrap() - chars.first().unwrap()
}

fn parse_input(input: &str) -> (String, HashMap<Key, char>) {
    let mut lines = input.lines();

    let polymer = lines.next().unwrap();
    lines.next();

    let transformations = lines
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .map(|line| {
            let mut pair = line[0].chars();

            (
                (pair.next().unwrap(), pair.next().unwrap()),
                line[1].chars().next().unwrap(),
            )
        })
        .collect();

    (polymer.to_string(), transformations)
}

mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        let mut polymer = HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
        let first = ('N', 'N');
        let last = ('C', 'B');

        assert_eq!(
            count_letters(polymer, first, last),
            HashMap::from([('N', 2), ('C', 1), ('B', 1)])
        );

        let mut polymer = HashMap::from([
            (('N', 'B'), 2),
            (('B', 'C'), 2),
            (('C', 'C'), 1),
            (('C', 'N'), 1),
            (('B', 'B'), 2),
            (('C', 'B'), 2),
            (('B', 'H'), 1),
            (('H', 'C'), 1),
        ]);
        //"NBCCNBBBCBHCB"

        let first = ('N', 'B');
        let last = ('C', 'B');

        assert_eq!(
            count_letters(polymer, first, last),
            HashMap::from([('N', 2), ('C', 4), ('B', 6), ('H', 1)])
        );
    }

    #[test]
    fn test_aggregate_step() {
        let mut polymer = HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]);
        let mut transformations = HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]);

        //"NCNBCHB"
        let step_1 = HashMap::from([
            (('N', 'C'), 1),
            (('C', 'N'), 1),
            (('N', 'B'), 1),
            (('B', 'C'), 1),
            (('C', 'H'), 1),
            (('H', 'B'), 1),
        ]);

        let polymer_1 = aggregate_step(polymer, &transformations);
        assert_eq!(polymer_1, step_1);

        //"NBCCNBBBCBHCB"
        let step_2 = HashMap::from([
            (('N', 'B'), 2),
            (('B', 'C'), 2),
            (('C', 'C'), 1),
            (('C', 'N'), 1),
            (('B', 'B'), 2),
            (('C', 'B'), 2),
            (('B', 'H'), 1),
            (('H', 'C'), 1),
        ]);

        let polymer_2 = aggregate_step(polymer_1, &transformations);
        assert_eq!(polymer_2, step_2);
    }

    #[test]
    fn test_step() {
        let polymer = "AB".to_string();
        let mut transformations = HashMap::new();
        transformations.insert(('A', 'B'), 'C');

        assert_eq!(step(polymer, &transformations), "ACB".to_string());
    }

    #[test]
    fn test_step_2() {
        let polymer = "NNCB".to_string();
        let mut transformations = HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]);

        let polymer_1 = step(polymer, &transformations);
        assert_eq!(polymer_1, "NCNBCHB".to_string());

        let polymer_2 = step(polymer_1, &transformations);
        assert_eq!(polymer_2, "NBCCNBBBCBHCB".to_string());

        let polymer_3 = step(polymer_2, &transformations);
        assert_eq!(polymer_3, "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string());
    }
}
