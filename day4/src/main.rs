fn main() {
    let mut lines = include_str!("input.txt").split('\n');

    let bingo_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    let mut numbers: Vec<u32> = vec![];
    let mut cards: Vec<BingoCard> = vec![];

    for line in lines {
        if numbers.len() == 25 {
            cards.push(BingoCard {
                id: cards.len() as u32,
                numbers: numbers.clone(),
            });
            numbers = vec![];
        } else {
            let mut split_line: Vec<u32> = line
                .split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect();

            numbers.append(&mut split_line);
        }
    }

    let mut last_number: u32 = 0;
    let mut unmarked_numbers: u32 = 0;
    let mut found = false;

    for i in 5..bingo_numbers.len() {
        if !found {
            last_number = bingo_numbers[i - 1];

            for card in &cards {
                if card.check_win(&bingo_numbers[0..i]) {
                    found = true;
                    unmarked_numbers = card.get_unmarked(&bingo_numbers[0..i]);
                    break;
                }
            }
        }
    }

    let part_1 = unmarked_numbers * last_number;
    dbg!(part_1);

    let mut active_cards = cards.clone();
    for i in 5..bingo_numbers.len() {
        if active_cards.len() > 1 {
            last_number = bingo_numbers[i];
            for card in &cards {
                if card.check_win(&bingo_numbers[0..i]) {
                    let card_id = card.id;
                    active_cards = active_cards
                        .clone()
                        .iter()
                        .filter(|&card| card.id != card_id)
                        .map(|card| card.clone())
                        .collect()
                }
            }
        }
    }

    let mut last_number: u32 = 0;
    let mut unmarked_numbers: u32 = 0;
    let mut found = false;

    for i in 5..bingo_numbers.len() {
        if !found {
            for card in &active_cards {
                if card.check_win(&bingo_numbers[0..i]) {
                    unmarked_numbers = card.get_unmarked(&bingo_numbers[0..i]);
                    last_number = bingo_numbers[i - 1];
                    found = true;
                    break;
                }
            }
        }
    }

    let part_2 = unmarked_numbers * last_number;
    dbg!(part_2);
}

#[derive(Debug, Clone)]
struct BingoCard {
    id: u32,
    numbers: Vec<u32>,
}

impl BingoCard {
    fn check_win(&self, nums: &[u32]) -> bool {
        if nums.len() < 5 {
            return false;
        }

        let winning_schemas = [
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
        ];

        for schema in winning_schemas {
            let mut correct = true;
            for n in schema {
                let checked_number = *self.numbers.get(n).unwrap();
                if !nums.contains(&checked_number) {
                    correct = false;
                }
            }

            if correct {
                return true;
            }
        }

        false
    }

    fn get_unmarked(&self, nums: &[u32]) -> u32 {
        self.numbers
            .iter()
            .fold(0, |acc, n| if nums.contains(n) { acc } else { acc + n })
    }
}
