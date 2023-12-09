use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use crate::AocDay;

#[derive(Default)]
pub struct Day4 {
    cards: Vec<Card>,
}

impl Day4 {

    pub fn new() -> Day4 {
        Day4 { ..Default::default() }
    }

}

impl AocDay for Day4 {

    fn info(&self) -> (u8, String) {
        (4, "Scratchcards".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.cards = input.iter().map(|s| Card::parse(s)).collect();
        true
    }

    fn part1(&self) -> String {
        self.cards.iter().map(Card::score).sum::<u32>().to_string()
    }

    fn part2(&self) -> String {
        let len = self.cards.len();
        let mut count = 0;
        let mut queue: VecDeque<&Card> = VecDeque::from_iter(self.cards.iter());
        while let Some(card) = queue.pop_front() {
            count += 1;
            let matching = card.matching();
            if matching > 0 {
                let start_idx: usize = card.id as usize;
                let end_idx = (start_idx + matching as usize).min(len);
                self.cards[start_idx..end_idx].iter().rev().for_each(|c| queue.push_front(c));
            }
        }
        count.to_string()
    }

}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn parse(input: &str) -> Self {
        let colon = input.find(':').unwrap();
        let pipe = input.find('|').unwrap();
        let id = input[5..colon].trim().parse::<u32>().unwrap();
        let winning_numbers: HashSet<_> = input[colon+1..pipe]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let numbers: Vec<_> = input[pipe+1..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Self { id, winning_numbers, numbers }
    }

    fn matching(&self) -> u32 {
        self.numbers.iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }

    fn score(&self) -> u32 {
        match self.matching() {
            0 => 0,
            n => 1 << (n - 1)
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        let winning_numbers = self.winning_numbers.iter()
            .map(|n| format!("{: >2}", n))
            .collect::<Vec<_>>()
            .join(" ");
        let numbers = self.numbers.iter()
            .map(|n| format!("{: >2}", n))
            .collect::<Vec<_>>()
            .join(" ");
        f.write_fmt(format_args!("Card {id}: {winning_numbers} | {numbers}"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                      Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                      Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                      Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                      Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                      Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example() {
        let day4 = init(EX);
        assert_eq!(day4.part1(), "13");
        assert_eq!(day4.part2(), "30");
    }

    fn init(input: &str) -> Day4 {
        let mut day = Day4::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
