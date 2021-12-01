fn main() -> anyhow::Result<()> {
    let input: Vec<u32> = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    let part_1 = count_incrementing_pairs(&input);
    let part_2 = count_incrementing_triplets(&input);

    dbg!(part_1);
    dbg!(part_2);

    Ok(())
}

fn count_incrementing_pairs(s: &[u32]) -> u32 {
    s.windows(2)
        .map(|pair| if pair[0] < pair[1] { 1 } else { 0 })
        .sum::<u32>()
}

fn count_incrementing_triplets(s: &[u32]) -> u32 {
    let triplets = s
        .windows(3)
        .map(|triplet| triplet[0] + triplet[1] + triplet[2])
        .collect::<Vec<u32>>();

    count_incrementing_pairs(&triplets)
}
