use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::AocDay;

#[derive(Default)]
pub struct Day2 {
    games: Vec<Game>,
}

impl Day2 {

    pub fn new() -> Day2 {
        Day2 { ..Default::default() }
    }

}

impl AocDay for Day2 {

    fn info(&self) -> (u8, String) {
        (2, "Cube Conundrum".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.games = input.iter().map(|s| Game::parse(s)).collect();
        true
    }

    fn part1(&self) -> String {
        let bag = Draw { red: 12, green: 13, blue: 14 };
        let mut sum: u32 = 0;
        for g in self.games.clone() {
            let num = g.num;
            if g.feasible(bag) {
                sum += num;
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut sum: u32 = 0;
        for g in self.games.clone() {
            sum += g.min_bag_power();
        }
        sum.to_string()
    }

}

#[derive(Clone)]
struct Game {
    num: u32,
    draws: Vec<Draw>,
}

impl Game {

    fn feasible(self, bag: Draw) -> bool {
        self.draws.iter().all(|d| d.feasible(bag))
    }

    fn min_bag_power(self) -> u32 {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for d in self.draws {
            red = max(red, d.red);
            green = max(green, d.green);
            blue = max(blue, d.blue);
        }
        red * green * blue
    }

    fn parse(input: &str) -> Self {
        let len = input.len();
        let space = input.find(" ").unwrap() + 1;
        let colon = input.find(":").unwrap();
        let num: u32 = input[space..colon].parse().unwrap();
        let mut draws = vec![];
        for draw_str in input[colon + 1..len].split(";") {
            draws.push(Draw::parse(draw_str));
        }
        Game { num, draws }
    }

}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let game = self.num;
        let draws = self.draws.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("; ");
        f.write_fmt(format_args!("Game {game}: {draws}"))
    }
}

#[derive(Clone, Copy)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {

    fn feasible(self, bag: Draw) -> bool {
        self.red <= bag.red &&
            self.green <= bag.green &&
            self.blue <= bag.blue
    }

    fn parse(input: &str) -> Draw {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for x in input.trim().split(", ") {
            let space = x.find(" ").unwrap();
            let count: u32 = x[0..space].parse().unwrap();
            match x.chars().nth(space + 1).unwrap() {
                'r' => red = count,
                'g' => green = count,
                'b' => blue = count,
                _ => {},
            }
        }

        Draw { red, green, blue }
    }

}

impl Display for Draw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (red, green, blue) = (self.red, self.green, self.blue);
        f.write_fmt(format_args!("{red} red, {green} green, {blue} blue"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example() {
        let day2 = init(EX);
        assert_eq!(day2.part1(), "8");
        assert_eq!(day2.part2(), "2286");
    }

    fn init(input: &str) -> Day2 {
        let mut day = Day2::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
