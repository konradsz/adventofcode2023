use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,         // |
    Horizontal,       // -
    NorthEast,        // L
    NorthWest,        // J
    SouthWest,        // 7
    SouthEast,        // F
    Ground,           // .
    StartingPosition, // S
    Inner,
    Outter,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn move_to((x, y): (usize, usize), dir: Direction) -> (usize, usize) {
    let (dx, dy) = match dir {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };
    let new_x = (x as i32) + dx;
    let new_y = (y as i32) + dy;

    (new_x as usize, new_y as usize)
}

fn turn(map: &[Vec<Tile>], (x, y): (usize, usize), dir: Direction) -> Direction {
    match map[y][x] {
        Tile::NorthEast => match dir {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => panic!("wrong dir"),
        },
        Tile::NorthWest => match dir {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => panic!("wrong dir"),
        },
        Tile::SouthWest => match dir {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            _ => panic!("wrong dir"),
        },
        Tile::SouthEast => match dir {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => panic!("wrong dir"),
        },
        _ => dir,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut map = Vec::new();
    let mut starting_position = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let row = line
            .chars()
            .map(|c| match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                '.' => Tile::Ground,
                'S' => Tile::StartingPosition,
                _ => panic!("invalid input"),
            })
            .collect::<Vec<_>>();

        if let Some(x) = row.iter().position(|&tile| tile == Tile::StartingPosition) {
            starting_position = (x, y);
        }
        map.push(row);
    }

    let mut current_position = starting_position;
    let mut current_dir = Direction::Up;

    for (dx, dy) in DIRS {
        let (starting_x, starting_y) = starting_position;
        let new_x = starting_x as i32 + dx;
        let new_y = starting_y as i32 + dy;
        if new_x > map[0].len() as i32 || new_x < 0 || new_y > map.len() as i32 || new_x < 0 {
            continue;
        }

        let t = map[new_y as usize][new_x as usize];

        match (dx, dy) {
            (0, -1) => {
                if t == Tile::Vertical || t == Tile::SouthEast || t == Tile::SouthWest {
                    current_dir = Direction::Up;
                    break;
                }
            }
            (1, 0) => {
                if t == Tile::Horizontal || t == Tile::SouthWest || t == Tile::NorthWest {
                    current_dir = Direction::Right;
                    break;
                }
            }
            (0, 1) => {
                if t == Tile::Vertical || t == Tile::NorthEast || t == Tile::NorthWest {
                    current_dir = Direction::Down;
                    break;
                }
            }
            (-1, 0) => {
                if t == Tile::Horizontal || t == Tile::NorthEast || t == Tile::SouthEast {
                    current_dir = Direction::Left;
                    break;
                }
            }
            _ => panic!("wrong direction"),
        }
    }

    let mut loop_positions = vec![];
    loop {
        loop_positions.push(current_position);
        current_position = move_to(current_position, current_dir);
        current_dir = turn(&map, current_position, current_dir);
        if current_position == starting_position {
            break;
        }
    }

    // assert_eq!(loop_positions.len() / 2, 6768);

    flood_fill(&mut map, &loop_positions);

    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if t == &Tile::Inner {
                count += 1;
            }
        }
    }

    for (y, row) in map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if loop_positions.contains(&(x, y)) {
                print!("@");
                continue;
            }
            let ch = match t {
                Tile::Vertical => '|',
                Tile::Horizontal => '-',
                Tile::NorthEast => 'L',
                Tile::NorthWest => 'J',
                Tile::SouthWest => '7',
                Tile::SouthEast => 'F',
                Tile::Ground => '.',
                Tile::StartingPosition => 'S',
                Tile::Inner => 'I',
                Tile::Outter => 'O',
            };
            print!("{}", ch);
        }
        println!();
    }

    println!("{count}");
}

fn flood_fill(map: &mut Vec<Vec<Tile>>, loop_positions: &[(usize, usize)]) {
    loop {
        let mut ground_pos = None;
        for (y, row) in map.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if t == &Tile::Ground {
                    ground_pos = Some((x, y));
                }
            }
        }

        let mut to_fill = VecDeque::new();
        if let Some(gp) = ground_pos {
            to_fill.push_back(gp);
        } else {
            break;
        }

        let mut current_cluster = vec![];
        let mut inner = true;
        // while !to_fill.is_empty() {
        while let Some(position) = to_fill.pop_front() {
            if current_cluster.contains(&position) {
                continue;
            }

            current_cluster.push(position);
            for (dx, dy) in DIRS {
                let new_x = position.0 as i32 + dx;
                let new_y = position.1 as i32 + dy;
                if new_y < 0
                    || new_y >= map.len() as i32
                    || new_x < 0
                    || new_x >= map[0].len() as i32
                {
                    inner = false;
                    continue;
                }

                if map[new_y as usize][new_x as usize] == Tile::Ground {
                    to_fill.push_back((new_x as usize, new_y as usize));
                } else {
                    if !loop_positions.contains(&(new_x as usize, new_y as usize)) {
                        inner = false;
                    }
                }
            }
        }

        for (x, y) in current_cluster {
            map[y][x] = if inner { Tile::Inner } else { Tile::Outter };
        }

        // for (y, row) in map.iter().enumerate() {
        //     for (x, t) in row.iter().enumerate() {
        //         let ch = match t {
        //             Tile::Vertical => '|',
        //             Tile::Horizontal => '-',
        //             Tile::NorthEast => 'L',
        //             Tile::NorthWest => 'J',
        //             Tile::SouthWest => '7',
        //             Tile::SouthEast => 'F',
        //             Tile::Ground => '.',
        //             Tile::StartingPosition => 'S',
        //             Tile::Inner => 'I',
        //             Tile::Outter => 'O',
        //         };
        //         print!("{}", ch);
        //     }
        //     println!();
        // }
    }
}
