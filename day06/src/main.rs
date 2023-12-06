// Input:
// Time:        51     92     68     90
// Distance:   222   2031   1126   1225

const INPUT: [(u64, u64); 4] = [(51, 222), (92, 2031), (68, 1126), (90, 1225)];

fn get_number_of_ways_to_beat_record(time: u64, record: u64) -> u64 {
    let mut sum = 0;
    for t in 1..time {
        let time_left = time - t;
        let distance = time_left * t;
        if distance > record {
            sum += 1;
        }
    }
    sum
}

fn part_1() -> u64 {
    INPUT
        .into_iter()
        .map(|(time, record)| get_number_of_ways_to_beat_record(time, record))
        .product()
}

fn part_2() -> u64 {
    let time = format!("{}{}{}{}", INPUT[0].0, INPUT[1].0, INPUT[2].0, INPUT[3].0)
        .parse()
        .unwrap();
    let record = format!("{}{}{}{}", INPUT[0].1, INPUT[1].1, INPUT[2].1, INPUT[3].1)
        .parse()
        .unwrap();
    get_number_of_ways_to_beat_record(time, record)
}

fn main() {
    assert_eq!(part_1(), 500346);
    assert_eq!(part_2(), 42515755);
}
