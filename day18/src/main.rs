fn part_1(input: &Vec<(&str, &str, &str)>) -> i64 {
    let mut segments = Vec::new();
    let mut pos_a: (i64, i64) = (0, 0);
    let mut length = 0;
    for (dir, steps, _) in input {
        let steps = steps.parse::<i64>().unwrap();
        length += steps;
        let pos_b = match *dir {
            "U" => (pos_a.0, pos_a.1 - steps),
            "R" => (pos_a.0 + steps, pos_a.1),
            "D" => (pos_a.0, pos_a.1 + steps),
            "L" => (pos_a.0 - steps, pos_a.1),
            _ => panic!("invalid input"),
        };
        segments.push((pos_a, pos_b));
        pos_a = pos_b;
    }

    calculate_area(segments, length)
}

fn part_2(input: &Vec<(&str, &str, &str)>) -> i64 {
    let mut segments = Vec::new();
    let mut pos_a: (i64, i64) = (0, 0);
    let mut length = 0;
    for (_, _, color) in input {
        let steps = i64::from_str_radix(&color[2..7], 16).unwrap();
        length += steps;
        let pos_b = match &color[7..8] {
            "0" => (pos_a.0 + steps, pos_a.1),
            "1" => (pos_a.0, pos_a.1 + steps),
            "2" => (pos_a.0 - steps, pos_a.1),
            "3" => (pos_a.0, pos_a.1 - steps),
            _ => panic!("invalid input"),
        };

        segments.push((pos_a, pos_b));
        pos_a = pos_b;
    }

    calculate_area(segments, length)
}

fn calculate_area(segments: Vec<((i64, i64), (i64, i64))>, length: i64) -> i64 {
    let sum: i64 = segments
        .iter()
        .map(|((xa, ya), (xb, yb))| xa * yb - xb * ya)
        .sum();

    (sum + length) / 2 + 1
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&input), 53300);
    assert_eq!(part_2(&input), 64294334780659);
}
