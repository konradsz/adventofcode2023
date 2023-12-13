fn get_mirrored_indexes(reflection_index: usize, len: usize) -> Vec<(usize, usize)> {
    (0..=reflection_index)
        .rev()
        .zip(reflection_index + 1..len)
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reflection {
    Row(usize),
    Column(usize),
}

fn find_horizontal_reflection(rows: &[Vec<u8>], skip: Option<Reflection>) -> Option<Reflection> {
    let len = rows.len();
    for reflection_index in 0..len - 1 {
        let indexes = get_mirrored_indexes(reflection_index, len);

        if indexes.iter().all(|(lhs, rhs)| rows[*lhs] == rows[*rhs]) {
            if let Some(Reflection::Row(skip)) = skip {
                if reflection_index != skip {
                    return Some(Reflection::Row(reflection_index));
                }
            } else {
                return Some(Reflection::Row(reflection_index));
            }
        }
    }

    None
}

fn find_vertical_reflection(columns: &[Vec<u8>], skip: Option<Reflection>) -> Option<Reflection> {
    let len = columns.len();
    for reflection_index in 0..len - 1 {
        let indexes = get_mirrored_indexes(reflection_index, len);

        if indexes
            .iter()
            .all(|(lhs, rhs)| columns[*lhs] == columns[*rhs])
        {
            if let Some(Reflection::Column(skip)) = skip {
                if reflection_index != skip {
                    return Some(Reflection::Column(reflection_index));
                }
            } else {
                return Some(Reflection::Column(reflection_index));
            }
        }
    }

    None
}

fn find_reflection(
    rows: &[Vec<u8>],
    columns: &[Vec<u8>],
    skip: Option<Reflection>,
) -> Option<Reflection> {
    let reflection = find_horizontal_reflection(rows, skip);
    if reflection.is_some() {
        reflection
    } else {
        find_vertical_reflection(columns, skip)
    }
}

fn reflection_with_smudge((rows, columns): (&[Vec<u8>], &[Vec<u8>])) -> usize {
    let reflection = find_reflection(rows, columns, None).unwrap();
    match reflection {
        Reflection::Row(n) => (n + 1) * 100,
        Reflection::Column(n) => n + 1,
    }
}

fn reflection_without_smudge((rows, columns): (&[Vec<u8>], &[Vec<u8>])) -> usize {
    let mut rows = rows.to_vec();
    let mut columns = columns.to_vec();

    let reflection_with_smudge = find_reflection(&rows, &columns, None);

    let width = columns.len();
    let height = rows.len();
    for x in 0..width {
        for y in 0..height {
            let smudge = rows[y][x];
            if smudge == b'.' {
                rows[y][x] = b'#';
                columns[x][y] = b'#';
            } else {
                rows[y][x] = b'.';
                columns[x][y] = b'.';
            }

            let reflection = find_reflection(&rows, &columns, reflection_with_smudge);
            if let Some(index) = reflection {
                match index {
                    Reflection::Row(n) => return (n + 1) * 100,
                    Reflection::Column(n) => return n + 1,
                }
            }
            rows[y][x] = smudge;
            columns[x][y] = smudge;
        }
    }
    panic!("reflection not found");
}

fn part_1(mirrors: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> usize {
    mirrors
        .iter()
        .map(|(rows, columns)| reflection_with_smudge((rows, columns)))
        .sum()
}

fn part_2(mirrors: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> usize {
    mirrors
        .iter()
        .map(|(rows, columns)| reflection_without_smudge((rows, columns)))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mirrors = input
        .split("\n\n")
        .map(|mirror| {
            let rows = mirror
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>();
            let columns = (0..rows[0].len())
                .map(|idx| {
                    rows.iter()
                        .map(|row| *row.get(idx).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (rows, columns)
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&mirrors), 33728);
    assert_eq!(part_2(&mirrors), 28235);
}
