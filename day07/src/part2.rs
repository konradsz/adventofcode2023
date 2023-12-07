use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Label {
    A,
    K,
    Q,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    J,
}

impl From<char> for Label {
    fn from(value: char) -> Self {
        match value {
            'A' => Label::A,
            'K' => Label::K,
            'Q' => Label::Q,
            'J' => Label::J,
            'T' => Label::T,
            '9' => Label::N9,
            '8' => Label::N8,
            '7' => Label::N7,
            '6' => Label::N6,
            '5' => Label::N5,
            '4' => Label::N4,
            '3' => Label::N3,
            '2' => Label::N2,
            _ => panic!("whoops"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand(String);

#[derive(Debug, PartialEq, Eq, Ord)]
struct HandAndBid(Hand, u64);

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self
            .get_type_with_jokers()
            .cmp(&other.get_type_with_jokers())
        {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                for (self_char, other_char) in self.0.chars().zip(other.0.chars()) {
                    let self_label: Label = self_char.into();
                    let other_label: Label = other_char.into();
                    match self_label.cmp(&other_label) {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Equal => continue,
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut chars: HashMap<char, u32> = HashMap::new();
        for c in self.0.chars() {
            let entry = chars.entry(c).or_insert(0);
            *entry += 1;
        }
        match chars.len() {
            1 => Type::FiveOfAKind,
            2 => {
                if chars.values().any(|v| *v == 4) {
                    Type::FourOfAKind
                } else {
                    Type::FullHouse
                }
            }
            3 => {
                if chars.values().any(|v| *v == 3) {
                    Type::ThreeOfAKind
                } else {
                    Type::TwoPair
                }
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => panic!("whoops"),
        }
    }

    fn get_type_with_jokers(&self) -> Type {
        let mut jokers = 0;
        let mut chars: HashMap<char, u32> = HashMap::new();
        for c in self.0.chars() {
            if c == 'J' {
                jokers += 1;
            } else {
                let entry = chars.entry(c).or_insert(0);
                *entry += 1;
            }
        }

        match jokers {
            5 | 4 => Type::FiveOfAKind,
            3 if chars.len() == 2 => Type::FourOfAKind,
            3 => Type::FiveOfAKind,
            2 => match chars.len() {
                3 => Type::ThreeOfAKind,
                2 => Type::FourOfAKind,
                _ => Type::FiveOfAKind,
            },
            1 => match chars.len() {
                4 => Type::OnePair,
                3 => Type::ThreeOfAKind,
                2 => {
                    if chars.values().any(|v| *v == 3) {
                        Type::FourOfAKind
                    } else {
                        Type::FullHouse
                    }
                }
                _ => Type::FiveOfAKind,
            },
            _ => self.get_type(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            HandAndBid(Hand(hand.into()), bid.parse().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort();

    let len = hands.len();

    let mut sum = 0;
    for (idx, HandAndBid(_, bid)) in hands.into_iter().enumerate() {
        sum += (len - idx) as u64 * bid;
    }

    println!("{sum}");
}
