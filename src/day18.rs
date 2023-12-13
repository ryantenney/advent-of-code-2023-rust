use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day18 {
}

impl Day18 {

    pub fn new() -> Day18 {
        Day18 { ..Default::default() }
    }

}

impl AocDay for Day18 {

    fn day(&self) -> u8 {
        18
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
        let day18 = init(EX);
        assert_eq!(day18.part1(), "");
    }

    fn init(input: &str) -> Day18 {
        let mut day = Day18::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
