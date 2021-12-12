use std::{collections::HashMap, path};

fn main() {
    let path_map: HashMap<String, Vec<String>> = get_path_map(include_str!("input.txt"));

    let part_1 = find_paths(&path_map, false);
    dbg!(part_1.len());

    let part_2 = find_paths(&path_map, true);
    dbg!(part_2.len());
}

fn get_path_map(input: &str) -> HashMap<String, Vec<String>> {
    let mut path_map: HashMap<String, Vec<String>> = HashMap::new();

    let item_pairs: Vec<(String, String)> = input
        .split('\n')
        .map(|line| {
            let path: Vec<String> = line.split('-').map(|s| s.to_string()).collect();
            (path[0].clone(), path[1].clone())
        })
        .collect();

    item_pairs.iter().for_each(|(a, b)| {
        path_map.entry(a.clone()).or_default().push(b.clone());
        path_map.entry(b.clone()).or_default().push(a.clone());
    });

    path_map
}

fn find_paths(path_map: &HashMap<String, Vec<String>>, visit_twice: bool) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    let mut incomplete_paths: Vec<Vec<String>> = vec![];

    for path in path_map.get("start").unwrap() {
        incomplete_paths.push(vec!["start".to_string(), path.clone()]);
    }

    while !incomplete_paths.is_empty() {
        let mut path = incomplete_paths.pop().unwrap();

        let last = path.last().unwrap();

        let next_paths = path_map.get(last).unwrap();

        next_paths
            .iter()
            .filter(|&next_path| {
                let is_big_cave = next_path != &next_path.clone().to_lowercase();

                if is_big_cave {
                    return true;
                }

                let cave_not_visited = !path.contains(&next_path);

                if cave_not_visited {
                    return true;
                }

                if visit_twice {
                    if next_path == "start" || next_path == "end" {
                        return false;
                    }

                    let small_caves = path
                        .iter()
                        .filter(|&cave| cave == &cave.clone().to_lowercase());

                    let is_another_small_cave_visited_twice = small_caves
                        .map(|cave| path.iter().filter(|&c| c == cave).count())
                        .any(|cave_occurences| cave_occurences == 2);

                    !is_another_small_cave_visited_twice
                } else {
                    false
                }
            })
            .for_each(|next_path| {
                let next_path = next_path.clone();
                let mut path = path.clone();

                path.push(next_path.clone());
                if next_path == "end" {
                    paths.push(path);
                } else {
                    incomplete_paths.push(path);
                }
            });
    }

    paths
}
