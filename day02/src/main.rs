struct Subset {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

type Game = Vec<Subset>;

fn part_1(games: &[Game]) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    games
        .iter()
        .enumerate()
        .map(|(idx, subsets)| {
            if subsets.iter().all(|Subset { red, green, blue }| {
                red.map_or(true, |count| count <= MAX_RED)
                    && green.map_or(true, |count| count <= MAX_GREEN)
                    && blue.map_or(true, |count| count <= MAX_BLUE)
            }) {
                (idx + 1) as u32
            } else {
                0
            }
        })
        .sum()
}

fn part_2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|subsets| {
            let red = subsets
                .iter()
                .map(|Subset { red, .. }| red.unwrap_or(0))
                .max()
                .unwrap();

            let green = subsets
                .iter()
                .map(|Subset { green, .. }| green.unwrap_or(0))
                .max()
                .unwrap();

            let blue = subsets
                .iter()
                .map(|Subset { blue, .. }| blue.unwrap_or(0))
                .max()
                .unwrap();

            red * green * blue
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut games = Vec::new();

    for line in input.lines() {
        let mut subsets = Vec::new();
        let (_, game) = line.split_once(": ").unwrap();
        for subset in game.split_terminator("; ") {
            let mut red = None;
            let mut green = None;
            let mut blue = None;
            for color in subset.split_terminator(", ") {
                if color.contains("red") {
                    let (count, _) = color.split_once(' ').unwrap();
                    red = Some(count.parse().unwrap());
                }

                if color.contains("green") {
                    let (count, _) = color.split_once(' ').unwrap();
                    green = Some(count.parse().unwrap());
                }

                if color.contains("blue") {
                    let (count, _) = color.split_once(' ').unwrap();
                    blue = Some(count.parse().unwrap());
                }
            }
            subsets.push(Subset { red, green, blue });
        }
        games.push(subsets);
    }

    assert_eq!(part_1(&games), 2265);
    assert_eq!(part_2(&games), 64097);
}
