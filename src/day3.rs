use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::AocDay;

#[derive(Default)]
pub struct Day3 {
    parts_with_neighbors: Vec<(Part, Vec<Neighbor>)>,
}

impl Day3 {

    pub fn new() -> Day3 {
        Day3 { ..Default::default() }
    }

}

impl AocDay for Day3 {

    fn info(&self) -> (u8, String) {
        (3, "Gear Ratios".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        let parts = find_parts(input);
        self.parts_with_neighbors = with_neighbors(parts, input);
        true
    }

    fn part1(&self) -> String {
        self.parts_with_neighbors.iter()
            .filter(|(_, neighbors)| !neighbors.is_empty())
            .map(|(part, _)| part.id)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let neighbors: Vec<_> = self.parts_with_neighbors.iter()
            .flat_map(|(part, neighbors)| {
                neighbors.iter()
                    .filter(|n| n.gear())
                    .map(move |n| (n, part))
            })
            .collect();

        let mut map: HashMap<&Neighbor, Vec<&Part>> = HashMap::new();
        for (neighbor, part) in neighbors {
            map.entry(neighbor).or_insert_with(Vec::new).push(part)
        }

        map.iter()
            .filter(|(_, parts)| parts.len() == 2)
            .map(|(_, parts)| {
                parts.iter()
                    .map(|p| p.id)
                    .reduce(|a, b| a * b)
                    .unwrap()
            })
            .sum::<u32>()
            .to_string()
    }

}

#[derive(Debug,Default)]
struct Part {
    id: u32,
    row: usize,
    col: usize,
    len: usize
}

impl Part {
    fn new(id: u32, row: usize, col: usize) -> Part {
        Part { id, row, col, len: 1 }
    }

    fn append(&mut self, digit: u32) {
        self.id = self.id * 10 + digit;
        self.len += 1;
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Neighbor {
    chr: char,
    row: usize,
    col: usize,
}

impl Neighbor {
    fn new(chr: char, row: usize, col: usize) -> Self {
        Self { chr, row, col }
    }

    fn valid(&self) -> bool {
        self.chr != '.' && !self.chr.is_ascii_digit()
    }

    fn gear(&self) -> bool {
        self.chr == '*'
    }
}

impl Display for Neighbor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let chr = self.chr;
        let row = self.row;
        let col = self.col.to_string();
        f.write_fmt(format_args!("({row}, {col}) = {chr}"))
    }
}

fn find_parts(input: &Vec<String>) -> Vec<Part> {
    let _parts: Vec<Part> = input.iter().enumerate()
        .flat_map(|(row, str)| {
            str.char_indices()
                .filter(|(_, chr)| chr.is_digit(10))
                .map(move |(col, chr)| {
                    Part::new(chr.to_digit(10).unwrap(), row, col)
                })
        })
        .collect();

    let mut output: Vec<Part> = vec![];
    let mut part: Option<Part> = None;
    for (row, str) in input.iter().enumerate() {
        for (col, chr) in str.char_indices() {
            if let Some(digit) = chr.to_digit(10) {
                if part.is_some() {
                    part.as_mut().unwrap().append(digit);
                } else {
                    part = Some(Part::new(digit, row, col))
                }
            } else if part.is_some() {
                output.push(part.unwrap());
                part = None
            }
        }

        if part.is_some() {
            output.push(part.unwrap());
            part = None
        }
    }
    output
}

fn with_neighbors(parts: Vec<Part>, input: &Vec<String>) -> Vec<(Part, Vec<Neighbor>)> {
    let rows = input.len();
    let cols = input.get(0).unwrap().len();
    let mut output: Vec<(Part, Vec<Neighbor>)> = Vec::with_capacity(parts.len());
    for part in parts {
        let min_row = part.row.checked_sub(1).unwrap_or(0);
        let max_row = (part.row + 1).min(rows - 1);
        let min_col = part.col.checked_sub(1).unwrap_or(0);
        let max_col = (part.col + part.len).min(cols - 1);
        let neighbors: Vec<Neighbor> = input[min_row..=max_row].iter().enumerate()
            .flat_map(|(row_offset, row_str)| {
                let row = min_row + row_offset;
                row_str.char_indices()
                    .filter(|(col, _)| *col >= min_col && *col <= max_col)
                    .map(move |(col, chr)| Neighbor::new(chr, row, col))
                    .filter(Neighbor::valid)
            })
            .collect();

        output.push((part, neighbors));
    }
    output
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "467..114..
                      ...*......
                      ..35..633.
                      ......#...
                      617*......
                      .....+.58.
                      ..592.....
                      ......755.
                      ...$.*....
                      .664.598..";

    #[test]
    fn example() {
        let day3 = init(EX);
        assert_eq!(day3.part1(), "4361");
        assert_eq!(day3.part2(), "467835");
    }

    fn init(input: &str) -> Day3 {
        let mut day = Day3::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
