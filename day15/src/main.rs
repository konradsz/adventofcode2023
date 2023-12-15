use std::collections::HashMap;

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |mut acc, c| {
        acc += c as u32;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part_1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn part_2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<String>> = HashMap::new();
    (0..256).for_each(|i| {
        boxes.insert(i, Vec::new());
    });

    for part in input.split(',') {
        if part.contains('-') {
            let label = &part[0..part.len() - 1];
            let box_n = hash(label);
            let lenses = boxes.get_mut(&box_n).unwrap();
            lenses.retain(|el| {
                let (l, _) = el.split_once('=').unwrap();
                label != l
            });
        } else if part.contains('=') {
            let label = &part[0..part.len() - 2];

            let box_n = hash(label);
            let lenses = boxes.get_mut(&box_n).unwrap();
            if let Some(idx) = lenses.iter().position(|el| {
                let (l, _) = el.split_once('=').unwrap();
                label == l
            }) {
                lenses.remove(idx);
                lenses.insert(idx, part.to_string());
            } else {
                lenses.push(part.to_string());
            }
        }
    }

    boxes
        .iter()
        .map(|(box_idx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot_idx, l)| {
                    (1 + box_idx)
                        * (1 + slot_idx as u32)
                        * l.chars().last().unwrap().to_digit(10).unwrap()
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(part_1(&input), 508498);
    assert_eq!(part_2(&input), 279116);
}
