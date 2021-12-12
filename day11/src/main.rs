use std::collections::HashMap;

fn main() {
    let mut octopuses: HashMap<(i32, i32), i32> = HashMap::new();
    include_str!("input.txt")
        .split('\n')
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .enumerate()
                .for_each(|(x, c)| {
                    octopuses.insert((x as i32, y as i32), c);
                });
        });

    let mut lighted = 0;

    for i in 1.. {
        let (new_octopus, lights, exploded) = step(octopuses);
        octopuses = new_octopus;
        lighted += lights;

        if (exploded == 100) {
            dbg!(i, exploded);
            break;
        }

        // display_grid(&octopuses);
    }

    dbg!(lighted);
}

fn display_grid(octopuses: &HashMap<(i32, i32), i32>) {
    print!("{}", '\n');
    for y in 0..10 {
        for x in 0..10 {
            print!("{} ", octopuses.get(&(x, y)).unwrap());
        }

        print!("{}", '\n');
    }
}

fn step(mut octopuses: HashMap<(i32, i32), i32>) -> (HashMap<(i32, i32), i32>, i32, usize) {
    let mut light_queue: Vec<(i32, i32)> = vec![];
    let mut exploded: Vec<(i32, i32)> = vec![];

    let mut lighted = 0;

    for (key, val) in octopuses.iter_mut() {
        *val += 1;
        if *val >= 10 {
            lighted += 1;
            light_queue.push(*key);
            exploded.push(*key);
        }
    }

    while !light_queue.is_empty() {
        let (x, y) = light_queue.pop().unwrap();
        let neighbours = [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
        ];

        neighbours.iter().for_each(|k| {
            if let Some(key) = octopuses.get_mut(k) {
                *key += 1;

                if *key >= 10 && !exploded.contains(k) {
                    lighted += 1;
                    exploded.push(*k);
                    light_queue.push(*k);
                }
            }
        });
    }

    for key in &exploded {
        octopuses.insert(*key, 0);
    }

    (octopuses, lighted, exploded.len())
}
