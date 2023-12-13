use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day13 {
}

impl Day13 {

    pub fn new() -> Day13 {
        Day13 { ..Default::default() }
    }

}

impl AocDay for Day13 {

    fn day(&self) -> u8 {
        13
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
        let day13 = init(EX);
        assert_eq!(day13.part1(), "");
    }

    fn init(input: &str) -> Day13 {
        let mut day = Day13::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
