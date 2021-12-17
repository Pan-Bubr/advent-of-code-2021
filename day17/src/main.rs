use std::{collections::HashSet, thread::current};

fn main() {
    let input_x = 88..=125;
    let input_y = -157..=-103;

    let mut target: HashSet<(i32, i32)> = HashSet::new();

    for x in input_x {
        for y in input_y.clone() {
            target.insert((x, y));
        }
    }

    let hits = hitting_velocities(&target);

    let part_1 = hits.iter().map(|&(_, height)| height).max().unwrap();

    dbg!(part_1);

    let part_2 = hits.len();

    dbg!(part_2);
}

fn hitting_velocities(target: &HashSet<(i32, i32)>) -> HashSet<((i32, i32), i32)> {
    let mut hitting: HashSet<((i32, i32), i32)> = HashSet::new();
    let last_target = (
        target.iter().map(|&(x, _)| x).max().unwrap(),
        target.iter().map(|&(_, y)| y).min().unwrap(),
    );

    for x in 0..=last_target.0 {
        for y in last_target.1..200 {
            let (will_hit, peak_height) = will_hit((x, y), &target, last_target);

            if will_hit {
                hitting.insert(((x, y), peak_height));
            }
        }
    }

    hitting
}

fn will_hit(
    initial_velocity: (i32, i32),
    target: &HashSet<(i32, i32)>,
    last_target: (i32, i32),
) -> (bool, i32) {
    let mut velocity = initial_velocity;
    let mut current_position = (0, 0);

    let mut peak_height = 0;

    while current_position.0 <= last_target.0 && current_position.1 >= last_target.1 {
        let (pos_x, pos_y) = current_position;
        let (v_x, v_y) = velocity;

        peak_height = peak_height.max(pos_y);

        if target.contains(&current_position) {
            return (true, peak_height);
        }

        current_position = (pos_x + v_x, pos_y + v_y);
        velocity = (
            match v_x.cmp(&0) {
                std::cmp::Ordering::Less => v_x + 1,
                std::cmp::Ordering::Equal => v_x,
                std::cmp::Ordering::Greater => v_x - 1,
            },
            v_y - 1,
        );
        if target.contains(&current_position) {
            return (true, peak_height);
        }
    }

    (false, 0)
}

mod tests {
    use super::*;

    #[test]
    fn test_will_hit() {
        let mut target: HashSet<(i32, i32)> = HashSet::new();

        for x in 20..=30 {
            for y in -10..=-5 {
                target.insert((x, y));
            }
        }

        let coords: Vec<(i32, i32)> = vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];

        let last_target = (20, -10);

        for coords in coords {
            if !will_hit(coords, &target, last_target).0 {
                dbg!(coords);
            }

            assert!(will_hit(coords, &target, last_target).0);
        }
        assert!(will_hit((7, 2), &target, last_target).0);
        assert!(will_hit((6, 3), &target, last_target).0);
        assert!(will_hit((9, 0), &target, last_target).0);
        assert!(will_hit((27, -5), &target, last_target).0);
        assert!(will_hit((23, -10), &target, last_target).0);
        assert!(will_hit((7, -1), &target, (20, -10)).0);
    }

    #[test]
    fn test_hitting_velocities() {
        let mut target: HashSet<(i32, i32)> = HashSet::new();
        let mut last_target = (0, 0);

        for x in 20..=30 {
            for y in -10..=-5 {
                target.insert((x, y));
                last_target = (x, y);
            }
        }

        let hitting = hitting_velocities(&target);

        assert!(hitting.contains(&((6, 9), 45)));
    }

    #[test]
    fn test_hitting_velocities_len() {
        let mut target: HashSet<(i32, i32)> = HashSet::new();
        let mut last_target = (0, 0);

        for x in 20..=30 {
            for y in -10..=-5 {
                target.insert((x, y));
                last_target = (x, y);
            }
        }

        let hitting = hitting_velocities(&target);

        assert_eq!(hitting.len(), 112);
    }
}
