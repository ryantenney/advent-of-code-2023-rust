use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day7 {
    hands: Vec<Hand>,
}

impl Day7 {

    pub fn new() -> Day7 {
        Day7 { ..Default::default() }
    }

}

impl AocDay for Day7 {

    fn info(&self) -> (u8, String) {
        (7, "Camel Cards".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.hands = input.iter()
            .map(|s| s.parse::<Hand>().unwrap())
            .collect();

        true
    }

    fn part1(&self) -> String {
        let mut hands = self.hands.clone();
        hands.sort_by(|h1, h2| {
            let cmp = h1.score().cmp(&h2.score());
            if cmp == Equal {
                h1.rank(h2)
            } else {
                cmp
            }
        });

        let mut rank = 0;
        let mut score = 0;
        for x in hands {
            rank += 1;
            score += x.bid * rank;
        }

        score.to_string()
    }

    fn part2(&self) -> String {
        let mut hands: Vec<_> = self.hands.iter()
            .map(|h| h.make_wild())
            .collect();

        hands.sort_by(|h1, h2| {
            let cmp = h1.score().cmp(&h2.score());
            if cmp == Equal {
                h1.rank(h2)
            } else {
                cmp
            }
        });

        let mut rank = 0;
        let mut score = 0;
        for x in hands {
            rank += 1;
            score += x.bid * rank;
        }

        score.to_string()
    }

}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn make_wild(&self) -> Hand {
        Hand {
            cards: self.cards.iter().map(|c| {
                match c {
                    Card::J => Card::WILD,
                    c => *c
                }
            }).collect(),
            bid: self.bid,
        }
    }

    fn score(&self) -> WinType {
        let mut counts = HashMap::new();
        for card in self.cards.iter() {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }

        let wild = counts.get(&Card::WILD).unwrap_or(&0);
        let counts: HashMap<_, _> = counts.iter()
            .filter(|(card, _)| **card != &Card::WILD)
            .filter(|(_, count)| **count > 1)
            .collect();

        if counts.is_empty() {
            // all single/wild cards
            match wild {
                0 => WinType::HighCard,
                1 => WinType::OnePair,
                2 => WinType::ThreeOfAKind,
                3 => WinType::FourOfAKind,
                4 => WinType::FiveOfAKind,
                5 => WinType::FiveOfAKind,
                _ => panic!("Invalid wild count: {}", wild),
            }
        } else if counts.len() == 1 {
            let (_, count) = counts.iter().next().unwrap();
            match *count + wild {
                2 => WinType::OnePair,
                3 => WinType::ThreeOfAKind,
                4 => WinType::FourOfAKind,
                5 => WinType::FiveOfAKind,
                _ => panic!("Invalid count: {}", count),
            }
        } else if counts.len() == 2 {
            let count = counts.iter().map(|(_, count)| **count).sum::<u32>();
            match count {
                4 => match wild {
                    0 => WinType::TwoPair,
                    1 => WinType::FullHouse,
                    _ => panic!("Invalid wild count: {}", wild),
                },
                5 => WinType::FullHouse,
                _ => panic!("Invalid count: {}", count),
            }
        } else {
            panic!("Invalid counts: {:?}", counts);
        }
    }

    fn zip<'a>(&'a self, other: &'a Hand) -> Vec<(&'a Card, &'a Card)> {
        self.cards.iter().zip(other.cards.iter()).collect()
    }

    fn rank(&self, other: &Hand) -> Ordering {
        self.zip(other).iter()
            .map(|(c1, c2)| c1.cmp(c2))
            .find(|&o| o != Equal)
            .unwrap_or(Equal)
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').unwrap();

        let cards: Vec<_> = cards_str.trim()
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Card>().unwrap())
            .collect();

        let bid = bid_str.trim().parse::<u32>().unwrap();

        Ok(Hand { cards, bid })
    }
}

#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    WILD = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Card::WILD => "*".to_string(),
            Card::_2 => "2".to_string(),
            Card::_3 => "3".to_string(),
            Card::_4 => "4".to_string(),
            Card::_5 => "5".to_string(),
            Card::_6 => "6".to_string(),
            Card::_7 => "7".to_string(),
            Card::_8 => "8".to_string(),
            Card::_9 => "9".to_string(),
            Card::T => "T".to_string(),
            Card::J => "J".to_string(),
            Card::Q => "Q".to_string(),
            Card::K => "K".to_string(),
            Card::A => "A".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Card::WILD => "*".to_string(),
            Card::_2 => "2".to_string(),
            Card::_3 => "3".to_string(),
            Card::_4 => "4".to_string(),
            Card::_5 => "5".to_string(),
            Card::_6 => "6".to_string(),
            Card::_7 => "7".to_string(),
            Card::_8 => "8".to_string(),
            Card::_9 => "9".to_string(),
            Card::T => "T".to_string(),
            Card::J => "J".to_string(),
            Card::Q => "Q".to_string(),
            Card::K => "K".to_string(),
            Card::A => "A".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Card::WILD),
            "2" => Ok(Card::_2),
            "3" => Ok(Card::_3),
            "4" => Ok(Card::_4),
            "5" => Ok(Card::_5),
            "6" => Ok(Card::_6),
            "7" => Ok(Card::_7),
            "8" => Ok(Card::_8),
            "9" => Ok(Card::_9),
            "T" => Ok(Card::T),
            "J" => Ok(Card::J),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            _ => Err(anyhow::anyhow!("Invalid card: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
enum WinType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl Display for WinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WinType::HighCard => write!(f, "High Card"),
            WinType::OnePair => write!(f, "One Pair"),
            WinType::TwoPair => write!(f, "Two Pair"),
            WinType::ThreeOfAKind => write!(f, "Three of a Kind"),
            WinType::FullHouse => write!(f, "Full House"),
            WinType::FourOfAKind => write!(f, "Four of a Kind"),
            WinType::FiveOfAKind => write!(f, "Five of a Kind"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "32T3K 765
                      T55J5 684
                      KK677 28
                      KTJJT 220
                      QQQJA 483";

    const EX2: &str = "2345A 1
                       Q2KJJ 13
                       Q2Q2Q 19
                       T3T3J 17
                       T3Q33 11
                       2345J 3
                       J345A 2
                       32T3K 5
                       T55J5 29
                       KK677 7
                       KTJJT 34
                       QQQJA 31
                       JJJJJ 37
                       JAAAA 43
                       AAAAJ 59
                       AAAAA 61
                       2AAAA 23
                       2JJJJ 53
                       JJJJ2 41";

    #[test]
    fn example() {
        let day7 = init(EX);
        assert_eq!(day7.part1(), "6440");
        assert_eq!(day7.part2(), "5905");
    }

    #[test]
    fn reddit_example() {
        let day7 = init(EX2);
        assert_eq!(day7.part1(), "6592");
        assert_eq!(day7.part2(), "6839");
    }

    #[test]
    fn test_zip() {
        let one = "KK677 28".parse::<Hand>().unwrap();
        let two = "KTJJT 220".parse::<Hand>().unwrap();
        let ordering = one.rank(&two);
    }

    fn init(input: &str) -> Day7 {
        let mut day = Day7::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
