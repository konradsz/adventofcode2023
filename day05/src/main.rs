use rayon::prelude::*;

#[derive(Debug)]
struct Map {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Map {
    fn affects(&self, number: u64) -> bool {
        (self.source_start..self.source_start + self.length).contains(&number)
    }

    fn map(&self, number: u64) -> u64 {
        let shift = number - self.source_start;
        self.destination_start + shift
    }
}

fn part_1(almanac: &[Vec<Map>], seeds: Vec<u64>) -> u64 {
    seeds
        .into_iter()
        .map(|mut seed| {
            for maps in almanac.iter() {
                for map in maps {
                    if map.affects(seed) {
                        seed = map.map(seed);
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap()
}

fn part_2(almanac: &[Vec<Map>], seeds: Vec<u64>) -> u64 {
    seeds
        .par_chunks(2)
        .map(|seed_range| {
            let start = seed_range[0];
            let length = seed_range[1];

            (start..start + length)
                .into_par_iter()
                .map(|mut seed| {
                    for maps in almanac.iter() {
                        for map in maps {
                            if map.affects(seed) {
                                seed = map.map(seed);
                                break;
                            }
                        }
                    }
                    seed
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut almanac = vec![];
    for part in parts {
        let maps = part
            .lines()
            .skip(1)
            .map(|line| {
                let mut ns = line.split_whitespace();
                Map {
                    destination_start: ns.next().unwrap().parse().unwrap(),
                    source_start: ns.next().unwrap().parse().unwrap(),
                    length: ns.next().unwrap().parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        almanac.push(maps);
    }

    assert_eq!(part_1(&almanac, seeds.clone()), 88151870);
    assert_eq!(part_2(&almanac, seeds.clone()), 2008785);
}
