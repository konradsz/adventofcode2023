use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Rounded,
    Cube,
    Empty,
}

fn tilt_north(platform: &mut Vec<Vec<Field>>) {
    for y in 1..platform.len() {
        for x in 0..platform[y].len() {
            if platform[y][x] == Field::Rounded {
                for j in (0..y).rev() {
                    if platform[j][x] == Field::Empty {
                        platform[j][x] = Field::Rounded;
                        platform[j + 1][x] = Field::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(platform: &mut [Vec<Field>]) {
    for x in 1..platform[0].len() {
        for row in platform.iter_mut() {
            if row[x] == Field::Rounded {
                for j in (0..x).rev() {
                    if row[j] == Field::Empty {
                        row[j] = Field::Rounded;
                        row[j + 1] = Field::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(platform: &mut Vec<Vec<Field>>) {
    for y in (0..platform.len() - 1).rev() {
        for x in 0..platform[y].len() {
            if platform[y][x] == Field::Rounded {
                for j in (y + 1)..platform.len() {
                    if platform[j][x] == Field::Empty {
                        platform[j][x] = Field::Rounded;
                        platform[j - 1][x] = Field::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(platform: &mut [Vec<Field>]) {
    for x in (0..platform[0].len() - 1).rev() {
        for row in platform.iter_mut() {
            if row[x] == Field::Rounded {
                for j in (x + 1)..row.len() {
                    if row[j] == Field::Empty {
                        row[j] = Field::Rounded;
                        row[j - 1] = Field::Empty;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn calculate_load(platform: &[Vec<Field>]) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let rounded_rocks = row.iter().filter(|&f| f == &Field::Rounded).count();
            rounded_rocks * (platform.len() - y)
        })
        .sum()
}

fn get_coords(platform: &[Vec<Field>]) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for (y, row) in platform.iter().enumerate() {
        for (x, f) in row.iter().enumerate() {
            if f == &Field::Rounded {
                coords.push((x, y));
            }
        }
    }

    coords
}

fn part_1(mut platform: Vec<Vec<Field>>) -> usize {
    tilt_north(&mut platform);
    calculate_load(&platform)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut platform = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Field::Rounded,
                    '#' => Field::Cube,
                    '.' => Field::Empty,
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(platform.clone()), 102497);

    let mut coords_seen = HashSet::new();

    let mut states = HashMap::new();
    let mut cycle_start = 0;
    let mut cycle_noticed = 0;
    let mut part_2 = 0;
    for i in 1.. {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        if let Some(v) = states.get(&get_coords(&platform)) {
            cycle_start = *v;
        } else {
            states.insert(get_coords(&platform), i);
        }
        if !coords_seen.insert(get_coords(&platform)) {
            cycle_noticed = i;
            break;
        }

        if i == 142 {
            part_2 = calculate_load(&platform);
        }
    }
    let _cycle_size = cycle_noticed - cycle_start;
    // println!("{}", cycle_start + (1000000000 - cycle_start) % cycle_size);

    assert_eq!(part_2, 105008);
}
