use crate::AocDay;

#[derive(Default)]
pub struct Day15 {
}

impl Day15 {

    pub fn new() -> Day15 {
        Day15 { ..Default::default() }
    }

}

impl AocDay for Day15 {

    fn day(&self) -> u8 {
        15
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
        let day15 = init(EX);
        assert_eq!(day15.part1(), "");
    }

    fn init(input: &str) -> Day15 {
        let mut day = Day15::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
