use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let (points, folds): (HashSet<(i32, i32)>, Vec<Fold>) = parse_input(include_str!("input.txt"));
    let mut new_paper = points;

    for f in folds {
        new_paper = fold_paper(&new_paper, f);
        dbg!(new_paper.len());
    }

    show_paper(&new_paper);
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(i32),
    Y(i32),
}

fn fold_paper(points: &HashSet<(i32, i32)>, fold: Fold) -> HashSet<(i32, i32)> {
    let mut new_points: HashSet<(i32, i32)> = HashSet::new();

    let max_x = points.iter().map(|(x, _)| *x).reduce(i32::max).unwrap();
    let max_y = points.iter().map(|(_, y)| *y).reduce(i32::max).unwrap();

    for (x, y) in points {
        match &fold {
            Fold::X(fold_x) => {
                let offset_from_middle = fold_x - max_x / 2;
                match x.cmp(fold_x) {
                    Ordering::Less => {
                        new_points.insert((*x, *y));
                    }
                    Ordering::Greater => {
                        new_points.insert((fold_x - (x - fold_x), *y));
                    }
                    _ => {}
                }
            }
            Fold::Y(fold_y) => {
                let offset_from_middle = fold_y - max_y / 2;
                match y.cmp(fold_y) {
                    Ordering::Less => {
                        new_points.insert((*x, *y));
                    }
                    Ordering::Greater => {
                        new_points.insert((*x, fold_y - (y - fold_y)));
                    }
                    _ => {}
                }
            }
        }
    }

    new_points
}

fn show_paper(points: &HashSet<(i32, i32)>) {
    let max_x = points.iter().map(|(x, _)| *x).reduce(i32::max).unwrap();
    let max_y = points.iter().map(|(_, y)| *y).reduce(i32::max).unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            match points.get(&(x, y)) {
                Some(_) => {
                    print!("█");
                }
                None => {
                    print!("░");
                }
            }
        }
        println!();
    }
    println!();
}

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Fold>) {
    let mut point_list = vec![];
    let mut folds = vec![];
    for line in input.lines() {
        if line.chars().next().is_some() && line.chars().next().unwrap().is_digit(10) {
            let coordinates: Vec<i32> = line
                .split(',')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect();
            let point = (coordinates[0], coordinates[1]);
            point_list.push(point);
        } else if line.contains("fold") {
            let line_number: i32 = line.split('=').collect::<Vec<&str>>()[1].parse().unwrap();

            if line.contains('x') {
                folds.push(Fold::X(line_number));
            } else {
                folds.push(Fold::Y(line_number));
            }
        }
    }

    let mut points = HashSet::new();

    for point in point_list {
        points.insert(point);
    }

    (points, folds)
}
