use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rocks,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut map = Vec::new();
    let mut starting_point = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Tile::GardenPlot),
                'S' => {
                    starting_point = (x, y);
                    row.push(Tile::GardenPlot);
                }
                '#' => row.push(Tile::Rocks),
                _ => panic!("invalid input"),
            }
        }
        map.push(row);
    }

    let mut visited_plots = HashSet::new();
    let mut cache = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((starting_point, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if !cache.insert((pos, steps)) {
            continue;
        }
        if steps == 64 {
            visited_plots.insert(pos);
            continue;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_pos = (pos.0 as i32 + dx, pos.1 as i32 + dy);
            if new_pos.0 < 0
                || new_pos.0 >= map[0].len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= map.len() as i32
            {
                continue;
            }

            if map[new_pos.1 as usize][new_pos.0 as usize] == Tile::GardenPlot {
                queue.push_back(((new_pos.0 as usize, new_pos.1 as usize), steps + 1));
            }
        }
    }

    assert_eq!(visited_plots.len(), 3820);
}
