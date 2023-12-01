use crate::AocDay;

#[derive(Default)]
pub struct Day19 {
}

impl Day19 {

    pub fn new() -> Day19 {
        Day19 { ..Default::default() }
    }

}

impl AocDay for Day19 {

    fn day(&self) -> u8 {
        19
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
        let day19 = init(EX);
        assert_eq!(day19.part1(), "");
    }

    fn init(input: &str) -> Day19 {
        let mut day = Day19::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
