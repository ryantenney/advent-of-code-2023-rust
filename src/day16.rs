use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day16 {
}

impl Day16 {

    pub fn new() -> Day16 {
        Day16 { ..Default::default() }
    }

}

impl AocDay for Day16 {

    fn day(&self) -> u8 {
        16
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
        let day16 = init(EX);
        assert_eq!(day16.part1(), "");
    }

    fn init(input: &str) -> Day16 {
        let mut day = Day16::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
