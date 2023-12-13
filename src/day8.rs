use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day8 {
    moves: Vec<Dir>,
    map: Map,
}

impl Day8 {

    pub fn new() -> Day8 {
        Day8 { ..Default::default() }
    }

}

impl AocDay for Day8 {

    fn info(&self) -> (u8, String) {
        (8, "Haunted Wasteland".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.moves = input[0].chars().map(Dir::from_char).collect();

        self.map = input[2..].iter()
            .map(|s| s.split_once("=").unwrap())
            .map(|(k, v)| {
                let next = v.trim().trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
                (k.trim().to_string(), (next.0.to_string(), next.1.to_string()))
            })
            .collect();

        true
    }

    fn part1(&self) -> String {
        let mut state = State::new("AAA", |x| x == "ZZZ", &self.map, &self.moves);
        let mut steps: u32 = 0;
        while !state.is_zzz() {
            for m in &self.moves {
                steps += 1;
                state.next(*m);
            }
        }
        steps.to_string()
    }

    fn part2(&self) -> String {
        let states: Vec<_> = self.map.keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| State::new(k, |x| x.ends_with('Z'), &self.map, &self.moves).run())
            .collect();

        lcm(states.as_slice()).to_string()
    }

}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn from_char(s: char) -> Dir {
        match s {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid direction: {}", s),
        }
    }
}

type Map = HashMap<String, (String, String)>;

struct State<'a> {
    start_location: &'a str,
    end_location: Box<dyn Fn(&'a str) -> bool>,
    location: &'a str,
    map: &'a Map,
    moves: &'a Vec<Dir>,
}

impl<'a> State<'a> {
    fn new(start_location: &'a str, end_location: fn(&str) -> bool, map: &'a Map, moves: &'a Vec<Dir>) -> State<'a> {
        State { start_location, end_location: Box::new(end_location), location: start_location, map, moves }
    }

    fn next(&mut self, next_move: Dir) {
        if let Some(map) = self.map.get(self.location) {
            self.location = match next_move {
                Dir::Left => map.0.as_str(),
                Dir::Right => map.1.as_str(),
            };
        } else {
            panic!("Invalid location: {}", self.location);
        }
    }

    fn run(&mut self) -> u64 {
        let mut steps: u64 = 0;
        loop {
            for m in self.moves {
                steps += 1;
                self.next(*m);
                if (self.end_location)(self.location) {
                    return steps
                }
            }
        }
    }

    fn is_zzz(&self) -> bool {
        self.location == "ZZZ"
    }

    fn is_z(&self) -> bool {
        self.location.ends_with("Z")
    }
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State({}, {})", self.start_location, self.location)
    }
}

fn lcm(values: &[u64]) -> u64 {
    let mut result = values[0];
    for &v in &values[1..] {
        result = result * v / gcd(result, v);
    }
    result
}

fn gcd(p0: u64, p1: u64) -> u64 {
    let mut a = p0;
    let mut b = p1;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "RL

                       AAA = (BBB, CCC)
                       BBB = (DDD, EEE)
                       CCC = (ZZZ, GGG)
                       DDD = (DDD, DDD)
                       EEE = (EEE, EEE)
                       GGG = (GGG, GGG)
                       ZZZ = (ZZZ, ZZZ)";

    const EX2: &str = "LLR

                       AAA = (BBB, BBB)
                       BBB = (AAA, ZZZ)
                       ZZZ = (ZZZ, ZZZ)";

    const EX3: &str = "LR

                       11A = (11B, XXX)
                       11B = (XXX, 11Z)
                       11Z = (11B, XXX)
                       22A = (22B, XXX)
                       22B = (22C, 22C)
                       22C = (22Z, 22Z)
                       22Z = (22B, 22B)
                       XXX = (XXX, XXX)";

    #[test]
    fn example() {
        let day8 = init(EX1);
        assert_eq!(day8.part1(), "2");

        let day8 = init(EX2);
        assert_eq!(day8.part1(), "6");

        let day8 = init(EX3);
        assert_eq!(day8.part2(), "6");
    }

    fn init(input: &str) -> Day8 {
        let mut day = Day8::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
