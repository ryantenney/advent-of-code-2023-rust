use crate::AocDay;

#[derive(Default)]
pub struct Day11 {
}

impl Day11 {

    pub fn new() -> Day11 {
        Day11 { ..Default::default() }
    }

}

impl AocDay for Day11 {

    fn day(&self) -> u8 {
        11
    }

    fn init(&mut self, _input: &Vec<String>) -> bool {
        false
    }

    fn part1(&self) -> String {
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "";

    #[test]
    fn example() {
        let day11 = init(EX);
        assert_eq!(day11.part1(), "");
    }

    fn init(input: &str) -> Day11 {
        let mut day = Day11::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
