fn main() {
    let lines: Vec<Vec<bool>> = include_str!("input.txt")
        .split('\n')
        .map(map_line_to_bool_vec)
        .collect();

    let (gamma, epsilon) = calculate_gamma_epsilon(&lines);

    let part_1 = gamma * epsilon;
    dbg!(part_1);

    let part_2 = calculate_oxygen_generator_rating(&lines) * calculate_co2_generator_rating(&lines);
    dbg!(part_2);
}

fn calculate_gamma_epsilon(lines: &[Vec<bool>]) -> (usize, usize) {
    let length = lines.len();
    let mut lines = lines.iter().peekable();
    let mut counter: Vec<usize> = lines.peek().unwrap().iter().map(|c| *c as usize).collect();

    lines.for_each(|line| {
        line.iter().enumerate().for_each(|(i, bit)| {
            let count = counter.get_mut(i).unwrap();
            *count += *bit as usize;
        })
    });

    let dominant_bits: String = counter
        .iter()
        .map(|&n| if n > (length / 2) { '1' } else { '0' })
        .collect();

    let gamma = usize::from_str_radix(&dominant_bits, 2).unwrap();

    let least_dominant_bits: String = counter
        .iter()
        .map(|&n| if n < (length / 2) { '1' } else { '0' })
        .collect();

    let epsilon = usize::from_str_radix(&least_dominant_bits, 2).unwrap();

    (gamma, epsilon)
}

fn calculate_oxygen_generator_rating(lines: &[Vec<bool>]) -> usize {
    let mut active_lines: Vec<Vec<bool>> = lines.iter().map(|v| v.clone()).collect();
    let mut bit_checked: usize = 0;

    while active_lines.len() > 1 {
        let mut count = 0;

        active_lines.iter().for_each(|line| {
            let bit = line.get(bit_checked).unwrap();
            count += *bit as usize;
        });

        let dominant_bit = count > (active_lines.len() - 1) / 2;

        active_lines = active_lines
            .iter()
            .filter(|&line| *line.get(bit_checked).unwrap() == dominant_bit)
            .map(|vec| vec.clone())
            .collect();

        bit_checked += 1;
    }

    bool_vector_to_usize(active_lines.get(0).unwrap())
}

fn calculate_co2_generator_rating(lines: &[Vec<bool>]) -> usize {
    let mut active_lines: Vec<Vec<bool>> = lines.iter().map(|v| v.clone()).collect();
    let mut bit_checked: usize = 0;

    while active_lines.len() > 1 {
        let mut count = 0;

        active_lines.iter().for_each(|line| {
            let bit = line.get(bit_checked).unwrap();
            count += *bit as usize;
        });

        let least_dominant_bit = count <= ((active_lines.len() - 1) / 2);

        active_lines = active_lines
            .iter()
            .filter(|&line| *line.get(bit_checked).unwrap() == least_dominant_bit)
            .map(|vec| vec.clone())
            .collect();

        bit_checked += 1;
    }

    bool_vector_to_usize(active_lines.get(0).unwrap())
}

fn map_line_to_bool_vec(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("Unexpected char"),
        })
        .collect()
}

fn bool_vector_to_usize(input: &[bool]) -> usize {
    let bool_string: String = input
        .iter()
        .map(|b| match b {
            true => '1',
            false => '0',
        })
        .collect();

    usize::from_str_radix(&bool_string, 2).unwrap()
}
