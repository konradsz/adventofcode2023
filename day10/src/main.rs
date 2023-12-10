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
        if new_x >= map[0].len() as i32 || new_x < 0 || new_y >= map.len() as i32 || new_y < 0 {
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

    assert_eq!(loop_positions.len() / 2, 6768); // Part 1
    assert_eq!(ray_cast(&map, &loop_positions), 351); // Part 2
}

fn ray_cast(map: &[Vec<Tile>], loop_positions: &[(usize, usize)]) -> usize {
    let mut count = 0;
    for (y, row) in map.iter().enumerate() {
        let mut hits = 0;
        for (x, t) in row.iter().enumerate() {
            if loop_positions.contains(&(x, y)) {
                if t == &Tile::NorthEast || t == &Tile::NorthWest || t == &Tile::Vertical {
                    hits += 1;
                }
            } else {
                if hits % 2 != 0 {
                    count += 1;
                }
            }
        }
    }

    count
}
