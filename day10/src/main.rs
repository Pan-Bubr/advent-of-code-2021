use core::panic;
use std::collections::HashMap;

fn main() {
    let input: Vec<(bool, String)> = include_str!("input.txt")
        .split('\n')
        .map(is_legal)
        .collect();

    let corruption_score: u64 = input
        .iter()
        .map(|(_, y)| match y.as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0,
        })
        .sum();

    let part_1 = corruption_score;
    dbg!(part_1);

    let mut incompleteness_score: Vec<u64> = input
        .iter()
        .filter(|(x, _)| *x)
        .map(|(_, s)| incompleteness_score(s))
        .collect();

    incompleteness_score.sort();

    let middle = incompleteness_score.len() / 2;
    let part_2 = incompleteness_score[middle];
    dbg!(part_2);
}

fn is_legal(input: &str) -> (bool, String) {
    let open = ['(', '[', '{', '<'];
    let mut matches: HashMap<char, char> = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');
    matches.insert('>', '<');

    let mut queue: Vec<char> = vec![];

    for c in input.chars() {
        if open.contains(&c) {
            queue.push(c);
        } else {
            match queue.pop() {
                Some(open_c) => {
                    if *matches.get(&c).unwrap() != open_c {
                        return (false, c.to_string());
                    }
                }
                None => {
                    return (false, "\n".to_string());
                }
            }
        }
    }
    (true, queue.into_iter().collect())
}

fn incompleteness_score(s: &String) -> u64 {
    s.chars()
        .rev()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Wrong character"),
        })
        .fold(0, |acc, n| acc * 5 + n)
}

mod tests {
    use super::*;
    #[test]
    fn test_is_legal() {
        assert_eq!(is_legal("([])"), (true, "".to_string()));
        assert_eq!(is_legal("{()()()}"), (true, "".to_string()));
        assert_eq!(is_legal("<([{}])>"), (true, "".to_string()));
        assert_eq!(is_legal("[<>({}){}[([])<>]]"), (true, "".to_string()));
        assert_eq!(is_legal("(((((((((())))))))))"), (true, "".to_string()));

        assert_eq!(is_legal("(]"), (false, "]".to_string()));
        assert_eq!(is_legal("{()()()>"), (false, ">".to_string()));
        assert_eq!(is_legal("(((()))}"), (false, "}".to_string()));
        assert_eq!(is_legal("<([]){()}[{}])"), (false, ")".to_string()));
    }

    #[test]
    fn test_incompleteness_score() {
        assert_eq!(incompleteness_score(&"{{[[({([".to_string()), 288957);
        assert_eq!(incompleteness_score(&"({<[{(".to_string()), 5566);

        // )}>]}) - 5566 total points.
        // }}>}>)))) - 1480781 total points.
        // ]]}}]}]}> - 995444 total points.
        // ])}> - 294 total points.
    }
}
