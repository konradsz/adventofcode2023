use itertools::Itertools;

#[derive(Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
    y: usize,
}

impl Number {
    fn get_adjacent_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        let start = if self.start > 0 {
            self.start - 1
        } else {
            self.start
        };
        let end = self.end + 1;

        for x in start..=end {
            if self.y > 0 {
                positions.push((x, self.y - 1));
            }
            positions.push((x, self.y + 1));
        }
        positions.push((start, self.y));
        positions.push((end, self.y));

        positions
    }

    fn is_part_number(&self, schematic: &[Vec<char>]) -> bool {
        let positions = self.get_adjacent_positions();

        positions.iter().any(|(x, y)| {
            if let Some(row) = schematic.get(*y) {
                if let Some(c) = row.get(*x) {
                    return !c.is_ascii_digit() && *c != '.';
                }
            }

            false
        })
    }

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let positions = self.get_adjacent_positions();
        positions.contains(&(x, y))
    }
}

fn part_1(schematic: &[Vec<char>], numbers: &[Number]) -> u32 {
    numbers
        .iter()
        .filter(|number| number.is_part_number(schematic))
        .map(|number| number.value)
        .sum()
}

fn part_2(schematic: &[Vec<char>], numbers: &[Number]) -> u32 {
    let mut sum = 0;
    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '*' {
                let mut adjacent_numbers = vec![];
                for number in numbers.iter() {
                    if number.is_adjacent(x, y) {
                        adjacent_numbers.push(number.value);
                    }
                }

                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let schematic = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut numbers = Vec::new();

    for (y, row) in schematic.iter().enumerate() {
        let mut x = 0;
        for (is_number, group) in &row.iter().group_by(|c| c.is_ascii_digit()) {
            let group = group.collect_vec();
            if is_number {
                let number = group
                    .iter()
                    .fold(0, |number, d| number * 10 + d.to_digit(10).unwrap());
                numbers.push(Number {
                    value: number,
                    start: x,
                    end: x + group.len() - 1,
                    y,
                });
            }
            x += group.len();
        }
    }

    assert_eq!(part_1(&schematic, &numbers), 525119);
    assert_eq!(part_2(&schematic, &numbers), 76504829);
}
