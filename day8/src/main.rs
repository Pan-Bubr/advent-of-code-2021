use itertools::Itertools;

fn main() {
    let input: Vec<(Vec<String>, Vec<String>)> = include_str!("input.txt")
        .split('\n')
        .map(parse_line)
        .collect();

    let part_1 = count_1478(&input);
    dbg!(part_1);

    let part_2: u32 = input
        .iter()
        .map(|(code, numbers)| get_line_value(numbers, decode_line(code)))
        .sum();

    dbg!(part_2);
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let split: Vec<&str> = line.split(" | ").collect();

    (
        split[0]
            .split_whitespace()
            .map(|str| str.chars().sorted().collect::<String>())
            .collect::<Vec<String>>(),
        split[1]
            .split_whitespace()
            .map(|str| str.chars().sorted().collect::<String>())
            .collect::<Vec<String>>(),
    )
}

fn count_1478(input: &[(Vec<String>, Vec<String>)]) -> u32 {
    input
        .iter()
        .map(|(_, b)| {
            b.iter()
                .filter(|&s| [2, 3, 4, 7].contains(&s.len()))
                .count() as u32
        })
        .sum()
}

fn get_line_value(line: &[String], dictionary: Dictionary) -> u32 {
    line.iter().fold(0, |acc, n| {
        let numeric_value = if n.eq(&dictionary.n_0) {
            0
        } else if n.eq(&dictionary.n_1) {
            1
        } else if n.eq(&dictionary.n_2) {
            2
        } else if n.eq(&dictionary.n_3) {
            3
        } else if n.eq(&dictionary.n_4) {
            4
        } else if n.eq(&dictionary.n_5) {
            5
        } else if n.eq(&dictionary.n_6) {
            6
        } else if n.eq(&dictionary.n_7) {
            7
        } else if n.eq(&dictionary.n_8) {
            8
        } else if n.eq(&dictionary.n_9) {
            9
        } else if n.eq(&dictionary.n_0) {
            0
        } else {
            panic!("unexpected value");
        };

        acc * 10 + numeric_value
    })
}

fn decode_line(line: &[String]) -> Dictionary {
    let all_segments = "abcdefg";
    let mut known_numbers = vec![];

    // Obvious numbers:
    let n_1 = line.iter().find(|&s| s.len() == 2).unwrap();
    let n_4 = line.iter().find(|&s| s.len() == 4).unwrap();
    let n_7 = line.iter().find(|&s| s.len() == 3).unwrap();
    let n_8 = line.iter().find(|&s| s.len() == 7).unwrap();

    known_numbers.push(n_1);
    known_numbers.push(n_4);
    known_numbers.push(n_7);
    known_numbers.push(n_8);

    // F is one of the letters in 1, the second one is C
    // 2 is the only one without the letter F
    let mut letter_f: char = '\0';
    let mut letter_c: char = '\0';
    n_1.chars().for_each(|c| {
        let occurences = line.iter().filter(|entry| entry.contains(c)).count();

        if occurences == 9 {
            letter_f = c;
        } else {
            letter_c = c;
        }
    });

    let n_2 = line.iter().find(|&s| !s.contains(letter_f)).unwrap();
    known_numbers.push(n_2);

    // 6 is the only 6-segment number missing C
    let n_6 = line
        .iter()
        .find(|&s| !s.contains(letter_c) && s.len() == 6)
        .unwrap();
    known_numbers.push(n_6);

    // 5 is the last unknown number that's missing C
    let n_5 = line
        .iter()
        .filter(|s| !known_numbers.contains(s))
        .find(|s| !s.contains(letter_c))
        .unwrap();
    known_numbers.push(n_5);

    // 5 is also missing E
    // 0 is the only unknown number that's missing E
    let letter_e: char = all_segments
        .chars()
        .find(|c| !n_5.contains(*c) && *c != letter_c)
        .unwrap();

    let n_0 = line
        .iter()
        .filter(|s| !known_numbers.contains(s))
        .find(|s| s.contains(letter_e))
        .unwrap();
    known_numbers.push(n_0);

    // 2 is missing B
    // 9 is the only unknown numbet that's missing B

    let letter_b: char = all_segments
        .chars()
        .find(|c| !n_2.contains(*c) && *c != letter_f)
        .unwrap();

    let n_9 = line
        .iter()
        .filter(|s| !known_numbers.contains(s))
        .find(|s| s.contains(letter_b))
        .unwrap();

    known_numbers.push(n_9);

    // 3 is the only unkown number
    let n_3 = line.iter().find(|s| !known_numbers.contains(s)).unwrap();

    Dictionary {
        n_0: n_0.to_string(),
        n_1: n_1.to_string(),
        n_2: n_2.to_string(),
        n_3: n_3.to_string(),
        n_4: n_4.to_string(),
        n_5: n_5.to_string(),
        n_6: n_6.to_string(),
        n_7: n_7.to_string(),
        n_8: n_8.to_string(),
        n_9: n_9.to_string(),
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Dictionary {
    n_0: String,
    n_1: String,
    n_2: String,
    n_3: String,
    n_4: String,
    n_5: String,
    n_6: String,
    n_7: String,
    n_8: String,
    n_9: String,
}

mod tests {
    use crate::Dictionary;

    #[test]
    fn decode_line() {
        let line = super::parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(
            super::decode_line(&line.0),
            Dictionary {
                n_0: "abcdeg".to_string(),
                n_1: "ab".to_string(),
                n_2: "acdfg".to_string(),
                n_3: "abcdf".to_string(),
                n_4: "abef".to_string(),
                n_5: "bcdef".to_string(),
                n_6: "bcdefg".to_string(),
                n_7: "abd".to_string(),
                n_8: "abcdefg".to_string(),
                n_9: "abcdef".to_string(),
            }
        );
    }

    #[test]
    fn get_line_value() {
        let line = super::parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        assert_eq!(
            super::get_line_value(&line.1, super::decode_line(&line.0)),
            5353
        );

        let line = super::parse_line(
            " be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        );
        assert_eq!(
            super::get_line_value(&line.1, super::decode_line(&line.0)),
            5353
        );
    }

    #[test]
    fn parse_line() {
        let line = "be cfbegad fgaecd | fdgacbe cefdb";

        assert_eq!(
            super::parse_line(line),
            (
                vec![
                    "be".to_string(),
                    "abcdefg".to_string(),
                    "acdefg".to_string()
                ],
                vec!["abcdefg".to_string(), "bcdef".to_string()]
            )
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            super::count_1478(&[(vec!["abc".to_string()], vec!["abcde".to_string()])]),
            0
        );
        assert_eq!(
            super::count_1478(&[(
                vec!["abc".to_string()],
                vec!["abcde".to_string(), "abc".to_string()]
            )]),
            1
        );
        assert_eq!(
            super::count_1478(&[(
                vec!["abc".to_string()],
                vec![
                    "ab".to_string(),
                    "abc".to_string(),
                    "abcd".to_string(),
                    "abcdefg".to_string()
                ]
            )]),
            4
        );
    }
    #[test]
    fn part_2() {}
}
