use crate::AocDay;

#[derive(Default)]
pub struct Day1 {
}

impl Day1 {

    pub fn new() -> Day1 {
        Day1 { ..Default::default() }
    }

}

impl AocDay for Day1 {

    fn day(&self) -> u8 {
        1
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
        let day1 = init(EX);
        assert_eq!(day1.part1(), "");
    }

    fn init(input: &str) -> Day1 {
        let mut day = Day1::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
