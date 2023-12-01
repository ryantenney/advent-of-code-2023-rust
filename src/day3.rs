use crate::AocDay;

#[derive(Default)]
pub struct Day3 {
}

impl Day3 {

    pub fn new() -> Day3 {
        Day3 { ..Default::default() }
    }

}

impl AocDay for Day3 {

    fn day(&self) -> u8 {
        3
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
        let day3 = init(EX);
        assert_eq!(day3.part1(), "");
    }

    fn init(input: &str) -> Day3 {
        let mut day = Day3::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
