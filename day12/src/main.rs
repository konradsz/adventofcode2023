fn is_valid2(record: &[u8], groups: &[u32]) -> bool {
    let mut damaged_groups = vec![];
    let mut current_len = 0;
    for condition in record {
        if *condition == b'?' {
            break;
        }

        if *condition == b'#' {
            current_len += 1;
        } else {
            if current_len > 0 {
                damaged_groups.push(current_len);
                current_len = 0;
            }
        }
    }
    if current_len > 0 {
        damaged_groups.push(current_len);
    }

    damaged_groups == groups
}

fn is_partially_valid(record: &[u8], groups: &[u32]) -> bool {
    let mut damaged_groups = vec![];
    let mut current_len = 0;
    for condition in record {
        if *condition == b'?' {
            break;
        }

        if *condition == b'#' {
            current_len += 1;
        } else {
            if current_len > 0 {
                damaged_groups.push(current_len);
                current_len = 0;
            }
        }
    }
    if current_len > 0 {
        damaged_groups.push(current_len);
    }

    if damaged_groups.len() > groups.len() {
        return false;
    } else {
        if damaged_groups.iter().zip(groups.iter()).any(|(a, b)| a > b) {
            return false;
        }
    }

    true
}

fn count_arrangements(input: &[u8], groups: &[u32]) -> u32 {
    if !is_partially_valid(input, groups) {
        return 0;
    }

    if !input.contains(&b'?') {
        if is_valid2(input, groups) {
            return 1;
        } else {
            return 0;
        }
    }

    let unknown = input.iter().position(|c| *c == b'?').unwrap();

    let mut input1 = input.to_vec();
    input1[unknown] = b'.';
    let mut input2 = input.to_vec();
    input2[unknown] = b'#';
    return count_arrangements(&input1, groups) + count_arrangements(&input2, groups);
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            count_arrangements(record.as_bytes(), &groups)
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 7110);
}
