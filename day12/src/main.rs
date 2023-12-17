use std::collections::HashMap;

fn dfs(
    cache: &mut HashMap<(usize, usize, u32), u64>,
    record: &[u8],
    groups: &[u32],
    ptr: usize,
    curr_group: usize,
    curr_size: u32,
) -> u64 {
    if let Some(v) = cache.get(&(ptr, curr_group, curr_size)) {
        return *v;
    }

    if ptr >= record.len() {
        if curr_group >= groups.len() {
            return 1;
        }

        if curr_group == groups.len() - 1 && groups[curr_group] == curr_size {
            return 1;
        }
        return 0;
    }

    match record[ptr] {
        b'.' => {
            if curr_size == 0 {
                return dfs(cache, record, groups, ptr + 1, curr_group, curr_size);
            }

            if curr_group >= groups.len() || curr_size != groups[curr_group] {
                return 0;
            }

            dfs(cache, record, groups, ptr + 1, curr_group + 1, 0)
        }
        b'#' => {
            if curr_group >= groups.len() || groups[curr_group] < curr_size + 1 {
                0
            } else {
                dfs(cache, record, groups, ptr + 1, curr_group, curr_size + 1)
            }
        }
        b'?' => {
            let mut possibilities = 0;

            if curr_size == 0 {
                let p = dfs(cache, record, groups, ptr + 1, curr_group, curr_size);
                cache.insert((ptr + 1, curr_group, curr_size), p);
                possibilities += p;
            }

            if curr_group < groups.len() && curr_size < groups[curr_group] {
                let p = dfs(cache, record, groups, ptr + 1, curr_group, curr_size + 1);
                cache.insert((ptr + 1, curr_group, curr_size + 1), p);
                possibilities += p;
            }

            if curr_group < groups.len() && curr_size == groups[curr_group] {
                let p = dfs(cache, record, groups, ptr + 1, curr_group + 1, 0);
                cache.insert((ptr + 1, curr_group + 1, 0), p);
                possibilities += p;
            }
            possibilities
        }
        _ => panic!("incorrect input"),
    }
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();
            dfs(&mut cache, record.as_bytes(), &groups, 0, 0, 0)
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (record, groups) = line.split_once(' ').unwrap();
            let mut groups = groups
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let record = format!("{record}?{record}?{record}?{record}?{record}");
            let initial_group = groups.clone();
            for _ in 0..4 {
                groups.append(&mut initial_group.clone());
            }

            let mut cache = HashMap::new();
            dfs(&mut cache, record.as_bytes(), &groups, 0, 0, 0)
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 7110);
    assert_eq!(part_2(&input), 1566786613613);
}
