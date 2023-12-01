use crate::AocDay;

#[derive(Default)]
pub struct Day6 {
}

impl Day6 {

    pub fn new() -> Day6 {
        Day6 { ..Default::default() }
    }

}

impl AocDay for Day6 {

    fn day(&self) -> u8 {
        6
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
        let day6 = init(EX);
        assert_eq!(day6.part1(), "");
    }

    fn init(input: &str) -> Day6 {
        let mut day = Day6::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
