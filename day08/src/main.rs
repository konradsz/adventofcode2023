use std::collections::HashMap;

fn part_1(instruction: &str, network: &HashMap<&str, (&str, &str)>) -> u64 {
    let mut current_node = "AAA";
    let mut steps = 0;

    for dir in instruction.chars().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        match dir {
            'L' => current_node = network.get(current_node).unwrap().0,
            'R' => current_node = network.get(current_node).unwrap().1,
            _ => panic!("invalid instruction"),
        }
        steps += 1;
    }

    steps
}

fn part_2(instruction: &str, network: &HashMap<&str, (&str, &str)>) -> u64 {
    let ghost_nodes = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    for mut current_node in ghost_nodes {
        let mut steps = 0;

        for dir in instruction.chars().cycle() {
            if current_node.ends_with('Z') {
                break;
            }

            match dir {
                'L' => current_node = network.get(current_node).unwrap().0,
                'R' => current_node = network.get(current_node).unwrap().1,
                _ => panic!("invalid instruction"),
            }
            steps += 1;
        }

        println!("{steps}");
    }

    // calculate LCM of all results
    15299095336639
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut network = HashMap::new();
    let (instruction, nodes) = input.split_once("\n\n").unwrap();
    for line in nodes.lines() {
        let from = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        network.insert(from, (left, right));
    }

    assert_eq!(part_1(instruction, &network), 16343);
    assert_eq!(part_2(instruction, &network), 15299095336639);
}
