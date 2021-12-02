fn main() {
    let directions: Vec<Direction> = include_str!("input.txt")
        .split('\n')
        .map(map_direction)
        .collect();

    let position_1 = calculate_position(&directions);
    let position_2 = calculate_position_with_aim(&directions);

    let part_1 = position_1.depth * position_1.distance;
    let part_2 = position_2.depth * position_2.distance;

    dbg!(part_1);
    dbg!(part_2);
}

fn map_direction(line: &str) -> Direction {
    let mut split = line.split(' ');

    let direction = split.next().unwrap();
    let value = split.next().unwrap().parse::<u32>().unwrap();

    return match direction {
        "down" => Direction::Down(value),
        "forward" => Direction::Forward(value),
        "up" => Direction::Up(value),
        _ => panic!("Unexpecter direction in input"),
    };
}

fn calculate_position(directions: &[Direction]) -> Position {
    let mut depth = 0;
    let mut distance = 0;

    for direction in directions {
        match direction {
            Direction::Down(n) => {
                depth += n;
            }
            Direction::Up(n) => {
                depth -= n;
            }
            Direction::Forward(n) => {
                distance += n;
            }
        }
    }

    Position { depth, distance }
}

fn calculate_position_with_aim(directions: &[Direction]) -> Position {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for direction in directions {
        match direction {
            Direction::Down(n) => {
                aim += n;
            }
            Direction::Up(n) => {
                aim -= n;
            }
            Direction::Forward(n) => {
                distance += n;
                depth += aim * n;
            }
        }
    }

    Position { depth, distance }
}

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

struct Position {
    depth: u32,
    distance: u32,
}
