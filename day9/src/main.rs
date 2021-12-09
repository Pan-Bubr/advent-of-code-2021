use std::{collections::HashMap, hash::Hash, ops::Index};

fn main() {
    let input: Vec<&str> = include_str!("input.txt").split('\n').collect();

    let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, heigth) in line.chars().enumerate() {
            height_map.insert((x as i32, y as i32), heigth.to_digit(10).unwrap() as i32);
        }
    }

    let part_1 = calculate_risk_levels(&height_map);
    dbg!(part_1);

    let mut basin_sizes = get_basins_sizes(&height_map);
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let part_2 = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    dbg!(part_2);
}

fn calculate_risk_levels(map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut risk_levels = 0;
    for ((x, y), heigth) in map {
        let mut low_point = true;

        [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)]
            .iter()
            .for_each(|neighbour_key| {
                if let Some(neighbour_value) = map.get(neighbour_key) {
                    if neighbour_value <= heigth {
                        low_point = false;
                    }
                }
            });

        if low_point {
            risk_levels += heigth + 1;
        }
    }

    risk_levels
}

fn get_basins_sizes(map: &HashMap<(i32, i32), i32>) -> Vec<u32> {
    let mut water_map: HashMap<(i32, i32), bool> = HashMap::new();

    for ((x, y), heigth) in map {
        if *heigth != 9 {
            water_map.insert((*x, *y), true);
        }
    }

    let mut basins = vec![];

    while water_map.values().into_iter().any(|v| *v) {
        let test = water_map.iter().find(|&(_, v)| *v).unwrap();
        let neighbours = find_neighbours(test.0, &water_map);

        basins.push(neighbours.len() as u32);

        neighbours.iter().for_each(|&(x, y)| {
            water_map.insert((x, y), false);
        });
    }

    basins
}

fn find_neighbours(key: &(i32, i32), map: &HashMap<(i32, i32), bool>) -> Vec<(i32, i32)> {
    let mut visited: Vec<(i32, i32)> = vec![];
    let mut queue: Vec<(i32, i32)> = vec![*key];

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        let check_queue = queue.clone();

        neighbours
            .iter()
            .filter(|(x, y)| {
                !visited.iter().any(|(v_x, v_y)| v_x == x && v_y == y)
                    && !check_queue.iter().any(|(v_x, v_y)| v_x == x && v_y == y)
            })
            .for_each(|key| {
                if map.get(key).is_some() {
                    queue.push(*key);
                }
            });
        visited.push((x, y));
    }

    visited
}

mod tests {
    use std::collections::HashMap;
    #[test]
    fn find_neighbours() {
        let mut map: HashMap<(i32, i32), bool> = HashMap::new();

        map.insert((0, 0), true);
        map.insert((0, 1), true);
        map.insert((0, 2), true);
        map.insert((1, 1), true);
        map.insert((1, 2), true);
        map.insert((2, 2), true);
        map.insert((3, 2), true);
        map.insert((4, 3), true);

        map.insert((20, 2), true);

        let key = (0, 0);

        assert_eq!(
            super::find_neighbours(&key, &map),
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (3, 2), (1, 1)]
        );
    }
}
