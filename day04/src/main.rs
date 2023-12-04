use std::collections::HashSet;

fn part_1(matches: &[usize]) -> u32 {
    matches
        .iter()
        .map(|count| {
            if *count > 0 {
                2_u32.pow((count - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(matches: &[usize]) -> u32 {
    let mut owned_cards = vec![1_u32; matches.len()];

    for i in 0..owned_cards.len() {
        let no_of_cards = owned_cards[i];
        let current_matches = matches[i];
        for j in 1..=current_matches {
            owned_cards[i + j] += no_of_cards;
        }
    }

    owned_cards.iter().sum::<u32>()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let matches = input
        .lines()
        .map(|line| {
            let mut parts = line.split(&[':', '|']);
            let (_, winning_numbers, owned_numbers) =
                (parts.next(), parts.next().unwrap(), parts.next().unwrap());

            let winning_numbers = winning_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let owned_numbers = owned_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            owned_numbers.intersection(&winning_numbers).count()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&matches), 24542);
    assert_eq!(part_2(&matches), 8736438);
}
