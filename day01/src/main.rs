fn get_calibration_value(line: &str) -> u32 {
    let first_digit = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let second_digit = line
        .chars()
        .rfind(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    first_digit * 10 + second_digit
}

fn get_digit(line: &str, words: &[String]) -> u32 {
    let mut first_idx = line.len();
    let mut first_digit = 0;
    for (d, digit_word) in words.iter().enumerate() {
        if let Some(idx) = line.find(digit_word) {
            if idx < first_idx {
                first_idx = idx;
                first_digit = (d as u32 % 9) + 1;
            }
        }
    }

    first_digit
}

fn part_1(input: &str) -> u32 {
    input.lines().map(get_calibration_value).sum()
}

fn part_2(input: &str) -> u32 {
    let digit_words = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ]
    .map(ToString::to_string);

    let digits_words_reversed: Vec<String> = digit_words
        .iter()
        .map(|d| d.chars().rev().collect())
        .collect();

    input
        .lines()
        .map(|line| {
            let reversed: String = line.chars().rev().collect();
            let first_digit = get_digit(line, &digit_words);
            let second_digit = get_digit(&reversed, &digits_words_reversed);
            first_digit * 10 + second_digit
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 53194);
    assert_eq!(part_2(&input), 54249);
}
