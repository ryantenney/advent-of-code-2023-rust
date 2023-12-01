use crate::AocDay;

#[derive(Default)]
pub struct Day7 {
}

impl Day7 {

    pub fn new() -> Day7 {
        Day7 { ..Default::default() }
    }

}

impl AocDay for Day7 {

    fn day(&self) -> u8 {
        7
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
        let day7 = init(EX);
        assert_eq!(day7.part1(), "");
    }

    fn init(input: &str) -> Day7 {
        let mut day = Day7::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
