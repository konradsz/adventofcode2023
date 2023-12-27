use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    RightSlope,
    DownSlope,
    LeftSlope,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::UpSlope,
                    '>' => Tile::RightSlope,
                    'v' => Tile::DownSlope,
                    '<' => Tile::LeftSlope,
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let starting_pos = (1, 0);
    let destination = (map[0].len() - 2, map.len() - 1);

    let mut queue: BinaryHeap<((usize, usize), usize, (usize, usize))> = BinaryHeap::new();
    queue.push((starting_pos, 0, starting_pos));

    let mut max_steps = 0;
    while let Some((position, steps, came_from)) = queue.pop() {
        if position == destination {
            max_steps = max_steps.max(steps);
            continue;
        }

        match map[position.1][position.0] {
            Tile::UpSlope => {
                let new_pos = (position.0, position.1 - 1);
                queue.push((new_pos, steps + 1, position));
                continue;
            }
            Tile::RightSlope => {
                let new_pos = (position.0 + 1, position.1);
                queue.push((new_pos, steps + 1, position));
                continue;
            }
            Tile::DownSlope => {
                let new_pos = (position.0, position.1 + 1);
                queue.push((new_pos, steps + 1, position));
                continue;
            }
            Tile::LeftSlope => {
                let new_pos = (position.0 - 1, position.1);
                queue.push((new_pos, steps + 1, position));
                continue;
            }
            _ => (),
        }

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let new_pos = (position.0 as i32 + dx, position.1 as i32 + dy);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }

            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if new_pos == came_from || new_pos.0 == map[0].len() || new_pos.1 == map.len() {
                continue;
            }

            match map[new_pos.1][new_pos.0] {
                Tile::Path => {
                    queue.push((new_pos, steps + 1, position));
                }
                Tile::UpSlope if (dx, dy) == (0, -1) => queue.push((new_pos, steps + 1, position)),
                Tile::RightSlope if (dx, dy) == (1, 0) => {
                    queue.push((new_pos, steps + 1, position));
                }
                Tile::DownSlope if (dx, dy) == (0, 1) => {
                    queue.push((new_pos, steps + 1, position));
                }
                Tile::LeftSlope if (dx, dy) == (-1, 0) => {
                    queue.push((new_pos, steps + 1, position));
                }
                _ => continue,
            }
        }
    }

    assert_eq!(max_steps, 2254);
}
