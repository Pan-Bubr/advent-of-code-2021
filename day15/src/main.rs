use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

type RiskMap = HashMap<(i32, i32), i32>;

fn main() {
    let now = Instant::now();
    let risk_map: RiskMap = parse_input(include_str!("input.txt"));
    println!("Parse time: {}", now.elapsed().as_millis());
    let part_1 = calculate_path(&risk_map);

    dbg!(part_1);

    let extended_risk_map: RiskMap = extend_risk_map(&risk_map);
    let part_2 = calculate_path(&extended_risk_map);
    dbg!(part_2);

    dbg!(now.elapsed().as_millis());
}

fn extend_risk_map(map: &RiskMap) -> RiskMap {
    let finish = (
        map.keys().map(|(x, _)| *x).max().unwrap() + 1,
        map.keys().map(|(_, y)| *y).max().unwrap() + 1,
    );

    let mut new_map = HashMap::new();

    for y in 0..finish.1 {
        for x in 0..finish.0 {
            for i in 0..5 {
                for j in 0..5 {
                    let key = (x + finish.0 * i, y + finish.1 * j);
                    let mut new_value = map.get(&(x, y)).unwrap() + i + j;
                    while new_value > 9 {
                        new_value -= 9;
                    }
                    new_map.insert(key, new_value);
                }
            }
        }
    }
    new_map
}

fn parse_input(input: &str) -> RiskMap {
    let mut risk_map: HashMap<(i32, i32), i32> = HashMap::new();

    input
        .split('\n')
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .map(|(x, char)| ((x as i32, y as i32), char.to_digit(10).unwrap() as i32))
                .collect::<Vec<((i32, i32), i32)>>()
        })
        .for_each(|(key, value)| {
            risk_map.insert(key, value);
        });

    risk_map
}

fn show_risk_map(map: &RiskMap) {
    let finish = (
        map.keys().map(|(x, _)| *x).max().unwrap(),
        map.keys().map(|(_, y)| *y).max().unwrap(),
    );

    for y in 0..=finish.1 {
        for x in 0..=finish.0 {
            let val = *map.get(&(x, y)).unwrap();
            if val == i32::MAX {
                print!("{} ", "_");
            } else {
                print!("{} ", val);
            }
        }
        println!();
    }

    println!();
}

fn calculate_path(map: &RiskMap) -> i32 {
    let start = (0, 0);

    let finish = (
        map.keys().map(|(x, _)| *x).max().unwrap(),
        map.keys().map(|(_, y)| *y).max().unwrap(),
    );

    let mut unvisited: HashSet<(i32, i32)> = map.keys().map(|c| *c).collect();
    let mut risk_map: RiskMap = HashMap::new();
    for &key in &unvisited {
        risk_map.insert(key, i32::MAX);
    }

    let mut queue: BinaryHeap<((i32, i32), i32)> = BinaryHeap::new();

    let mut current_node = (0, 0);
    risk_map.insert(current_node, 0);

    while !unvisited.is_empty() {
        let (x, y) = current_node;

        if current_node == finish {
            return *risk_map.get(&current_node).unwrap();
        }

        let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        let current_node_value = *risk_map.get(&current_node).unwrap();

        for key in neighbours {
            if unvisited.contains(&key) && map.contains_key(&key) {
                let mut new_value = *map.get(&key).unwrap() + current_node_value;

                if let Some(value) = risk_map.get_mut(&key) {
                    new_value = new_value.min(*value);
                    *value = new_value;
                } else {
                    risk_map.insert(key, new_value);
                }

                queue.push((key, new_value));
            }
        }

        if unvisited.len() % 10000 == 0 {
            dbg!(unvisited.len());
        }

        unvisited.remove(&current_node);

        current_node = queue.pop().unwrap().0;
    }

    0
}
