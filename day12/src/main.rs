use std::collections::VecDeque;

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

fn is_valid(record: &[u8], idx: usize, groups: &[u32]) -> bool {
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

    // println!(
    //     "{} -> {:?}",
    //     String::from_utf8(record.to_vec()).unwrap(),
    //     damaged_groups
    // );

    // let found_groups = damaged_groups.len();
    // if let Some(to_check) = found_groups.checked_sub(1) {
    //     for i in 0..to_check {
    //         if damaged_groups[i] != groups[i] {
    //             return false;
    //         }
    //     }
    // }

    // if found_groups > 0 {
    //     if damaged_groups[found_groups - 1] > groups[found_groups - 1] {
    //         return false;
    //     }
    // }

    // true
    let mut result = true;
    if damaged_groups.len() > groups.len() {
        result = false;
    } else {
        if damaged_groups.iter().zip(groups.iter()).any(|(a, b)| a > b) {
            result = false;
        }
    }

    // if result {
    //     println!(
    //         "{} -> {:?} is valid",
    //         String::from_utf8(record.to_vec()).unwrap(),
    //         damaged_groups
    //     );
    // } else {
    //     println!(
    //         "{} -> {:?} is invalid",
    //         String::from_utf8(record.to_vec()).unwrap(),
    //         damaged_groups
    //     );
    // }

    result
}

fn main() {
    // let input = "?###????????";
    // let unknowns = input
    //     .bytes()
    //     .enumerate()
    //     .filter_map(|(idx, b)| if b == b'?' { Some(idx) } else { None })
    //     .collect::<Vec<_>>();

    // println!("{unknowns:?}");

    // let mut queue = VecDeque::new();
    // queue.push_back(("?###????????".as_bytes().to_vec(), unknowns));

    // while let Some((record, indexes)) = queue.pop_front() {
    //     // println!("{}", String::from_utf8(record.clone()).unwrap());
    //     if indexes.len() > 0 {
    //         if is_valid(&record, indexes[0], &[3, 2, 1]) {
    //             // println!("valid");
    //             let mut record1 = record.clone();
    //             record1[indexes[0]] = b'.';
    //             queue.push_back((record1.to_vec(), indexes[1..].to_vec()));

    //             let mut record2 = record;
    //             record2[indexes[0]] = b'#';
    //             queue.push_back((record2, indexes[1..].to_vec()));
    //         }
    //     } else {
    //         if is_valid2(&record, &[3, 2, 1]) {
    //             println!("{} is VALID", String::from_utf8(record.clone()).unwrap());
    //         }
    //     }
    // }
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let (record, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let unknowns = record
            .bytes()
            .enumerate()
            .filter_map(|(idx, b)| if b == b'?' { Some(idx) } else { None })
            .collect::<Vec<_>>();

        let mut queue = VecDeque::new();
        queue.push_back((record.as_bytes().to_vec(), unknowns));

        while let Some((record, indexes)) = queue.pop_front() {
            // println!("{}", String::from_utf8(record.clone()).unwrap());
            if indexes.len() > 0 {
                if is_valid(&record, indexes[0], &groups) {
                    // println!("valid");
                    let mut record1 = record.clone();
                    record1[indexes[0]] = b'.';
                    queue.push_back((record1.to_vec(), indexes[1..].to_vec()));

                    let mut record2 = record;
                    record2[indexes[0]] = b'#';
                    queue.push_back((record2, indexes[1..].to_vec()));
                }
            } else {
                if is_valid2(&record, &groups) {
                    // println!("{} is VALID", String::from_utf8(record.clone()).unwrap());
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");
}
