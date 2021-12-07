fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

fn calculate_fuel(from: u32, to: u32) -> u32 {
    from.max(to) - from.min(to)
}

fn calculate_fuel_incremental(from: u32, to: u32) -> u32 {
    let n = from.max(to) - from.min(to);

    (1 + n) * n / 2
}

fn part_1(input: &[u32]) -> u32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut minimum_fuel: u32 = u32::MAX;

    for possible_position in min..=max {
        let fuel: u32 = input
            .iter()
            .map(|&position| calculate_fuel(position, possible_position))
            .sum();

        minimum_fuel = fuel.min(minimum_fuel);
    }

    minimum_fuel
}

fn part_2(input: &[u32]) -> u32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut minimum_fuel: u32 = u32::MAX;

    for possible_position in min..=max {
        let fuel: u32 = input
            .iter()
            .map(|&position| calculate_fuel_incremental(position, possible_position))
            .sum();

        minimum_fuel = fuel.min(minimum_fuel);
    }

    minimum_fuel
}

#[cfg(test)]
mod tests {
    static input: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn calculate_fuel() {
        assert_eq!(super::calculate_fuel(10, 0), 10);
        assert_eq!(super::calculate_fuel(8, 10), 2);
        assert_eq!(super::calculate_fuel(5, 5), 0);
    }
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&input), 37);
    }
    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&input), 168);
    }
}
