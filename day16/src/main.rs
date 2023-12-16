use std::collections::HashSet;

enum Tile {
    Empty,              // .
    VerticalSplitter,   // |
    HorizontalSplitter, // -
    LeftMirror,         // /
    RightMirror,        // \
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
struct Beam {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Beam {
    fn mov(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn turn_on_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Up,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Down,
        }
    }

    fn turn_on_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Up,
        }
    }

    fn split_horizontally(&self) -> Vec<Beam> {
        match self.dir {
            Direction::Up | Direction::Down => {
                vec![
                    Beam {
                        dir: Direction::Left,
                        ..*self
                    },
                    Beam {
                        dir: Direction::Right,
                        ..*self
                    },
                ]
            }
            Direction::Right | Direction::Left => vec![*self],
        }
    }

    fn split_vertically(&self) -> Vec<Beam> {
        match self.dir {
            Direction::Right | Direction::Left => vec![
                Beam {
                    dir: Direction::Up,
                    ..*self
                },
                Beam {
                    dir: Direction::Down,
                    ..*self
                },
            ],
            Direction::Up | Direction::Down => vec![*self],
        }
    }

    fn is_outside(&self, width: usize, height: usize) -> bool {
        self.x < 0 || self.y < 0 || self.x >= width as i32 || self.y >= height as i32
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '|' => Tile::VerticalSplitter,
                    '-' => Tile::HorizontalSplitter,
                    '/' => Tile::LeftMirror,
                    '\\' => Tile::RightMirror,
                    _ => panic!("incorrect input"),
                })
                .collect()
        })
        .collect();

    let width = map[0].len();
    let height = map.len();

    let mut beams = vec![Beam {
        x: 0,
        y: 0,
        dir: Direction::Right,
    }];

    let mut energized_tiles = HashSet::new();

    let mut steps = 0;

    while steps < 600 {
        steps += 1;
        for beam in beams.iter() {
            energized_tiles.insert((beam.x, beam.y));
        }

        let mut new_beams = vec![];
        for beam in beams.iter_mut() {
            match map[beam.y as usize][beam.x as usize] {
                Tile::VerticalSplitter => new_beams.append(&mut beam.split_vertically()),
                Tile::HorizontalSplitter => new_beams.append(&mut beam.split_horizontally()),
                Tile::LeftMirror => {
                    beam.turn_on_left();
                    new_beams.push(*beam);
                }
                Tile::RightMirror => {
                    beam.turn_on_right();
                    new_beams.push(*beam);
                }
                _ => new_beams.push(*beam),
            }
        }

        beams = new_beams;

        for beam in beams.iter_mut() {
            beam.mov();
        }

        beams.retain(|beam| !beam.is_outside(width, height));
    }

    println!("{}", energized_tiles.len());
}
