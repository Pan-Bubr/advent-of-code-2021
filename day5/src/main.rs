use std::collections::HashMap;

fn main() {
    let lines: Vec<Line> = include_str!("input.example.txt")
        .split('\n')
        .map(map_line)
        .collect();

    let part_1 = find_overlapping_lines(&lines, false);
    let part_2 = find_overlapping_lines(&lines, true);

    dbg!(part_1, part_2);
}

fn find_overlapping_lines(lines: &[Line], horizontal: bool) -> u32 {
    let mut lines_map: HashMap<(u32, u32), u32> = HashMap::new();

    lines.iter().for_each(|line| {
        let from = line.from;
        let to = line.to;
        if from.0 == to.0 {
            for y in from.1.min(to.1)..=from.1.max(to.1) {
                *lines_map.entry((from.0, y)).or_default() += 1;
            }
        } else if from.1 == to.1 {
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                *lines_map.entry((x, from.1)).or_default() += 1;
            }
        } else if horizontal {
            if from.0 <= to.0 && from.1 <= to.1 {
                for (x, y) in (from.0..=to
                    .0).zip(from.1..=to.1) {
                    *lines_map.entry((x, y)).or_default() += 1;
                }
            } else if from.0 > to.0 && from.1 <= to.1 {
                for (x, y) in ((to.0..=from.0).rev()).zip(from.1..=to.1) {
                    *lines_map.entry((x, y)).or_default() += 1;
                }
            } else if from.0 <= to.0 && from.1 > to.1 {
                for (x, y) in (from.0..=to.0).zip((to.1..=from.1).rev()) {
                    *lines_map.entry((x, y)).or_default() += 1;
                }
            } else if from.0 > to.0 && from.1 > to.1 {
                for (x, y) in ((to.0..=from.0).rev()).zip((to.1..=from.1).rev()) {
                    *lines_map.entry((x, y)).or_default() += 1;
                }
            }
        }
    });

    lines_map
        .values()
        .fold(0, |acc, &n| if n >= 2 { acc + 1 } else { acc })
}

#[derive(Debug)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

fn map_line(line: &str) -> Line {
    let coordinates: Vec<&str> = line.split(" -> ").collect();

    let from: Vec<u32> = coordinates[0]
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    let to: Vec<u32> = coordinates[1]
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    Line {
        from: (from[0], from[1]),
        to: (to[0], to[1]),
    }
}
