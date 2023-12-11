fn empty_rows(universe: &[Vec<char>]) -> Vec<usize> {
    universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| c == &'.'))
        .map(|(idx, _)| idx)
        .collect()
}

fn empty_columns(universe: &[Vec<char>]) -> Vec<usize> {
    (0..universe[0].len())
        .filter(|idx| universe.iter().all(|row| row[*idx] == '.'))
        .collect()
}

fn count_empty_between(lhs: usize, rhs: usize, empty: &[usize]) -> usize {
    empty.iter().filter(|&idx| *idx > lhs && *idx < rhs).count()
}

fn sum_distances(universe: &[Vec<char>], expansion_rate: usize) -> i64 {
    let empty_rows = empty_rows(&universe);
    let empty_columns = empty_columns(&universe);

    let mut galaxies = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (g1_x, g1_y) = galaxies[i];
            let (g2_x, g2_y) = galaxies[j];

            let rows_to_insert = count_empty_between(g1_y.min(g2_y), g1_y.max(g2_y), &empty_rows);
            let columns_to_insert =
                count_empty_between(g1_x.min(g2_x), g1_x.max(g2_x), &empty_columns);

            let rows_to_insert = rows_to_insert * (expansion_rate - 1);
            let columns_to_insert = columns_to_insert * (expansion_rate - 1);

            let distance = ((g1_x.max(g2_x) + columns_to_insert) as i64 - g1_x.min(g2_x) as i64)
                + ((g1_y.max(g2_y) + rows_to_insert) as i64 - g1_y.min(g2_y) as i64);
            sum += distance;
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let universe: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    assert_eq!(sum_distances(&universe, 2), 9974721); // Part 1
    assert_eq!(sum_distances(&universe, 1000000), 702770569197); // Part 2
}
