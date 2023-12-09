fn extrapolate(history: &[i64]) -> i64 {
    let mut sequences = vec![history.to_vec()];
    generate_sequence(history, &mut sequences);

    sequences.iter().map(|s| s.last().unwrap()).sum()
}

fn extrapolate_backwards(history: &[i64]) -> i64 {
    let mut sequences = vec![history.to_vec()];
    generate_sequence(history, &mut sequences);

    let first_numbers = sequences
        .iter()
        .map(|s| s.first().unwrap())
        .collect::<Vec<_>>();

    first_numbers.iter().rev().fold(0, |acc, &w| w - acc)
}

fn generate_sequence(history: &[i64], sequences: &mut Vec<Vec<i64>>) {
    if history.iter().all(|s| *s == 0) {
        return;
    } else {
        let next_history = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

        sequences.push(next_history.clone());
        generate_sequence(&next_history, sequences);
    }
}

fn part_1(history_log: &[Vec<i64>]) -> i64 {
    history_log.iter().map(|history| extrapolate(history)).sum()
}

fn part_2(history_log: &[Vec<i64>]) -> i64 {
    history_log
        .iter()
        .map(|history| extrapolate_backwards(history))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let history_log = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&history_log), 1637452029);
    assert_eq!(part_2(&history_log), 908);
}
