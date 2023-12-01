use crate::AocDay;

#[derive(Default)]
pub struct Day10 {
}

impl Day10 {

    pub fn new() -> Day10 {
        Day10 { ..Default::default() }
    }

}

impl AocDay for Day10 {

    fn day(&self) -> u8 {
        10
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
        let day10 = init(EX);
        assert_eq!(day10.part1(), "");
    }

    fn init(input: &str) -> Day10 {
        let mut day = Day10::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}