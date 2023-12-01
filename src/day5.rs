use crate::AocDay;

#[derive(Default)]
pub struct Day5 {
}

impl Day5 {

    pub fn new() -> Day5 {
        Day5 { ..Default::default() }
    }

}

impl AocDay for Day5 {

    fn day(&self) -> u8 {
        5
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
        let day5 = init(EX);
        assert_eq!(day5.part1(), "");
    }

    fn init(input: &str) -> Day5 {
        let mut day = Day5::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
