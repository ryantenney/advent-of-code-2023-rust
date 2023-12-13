use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day12 {
}

impl Day12 {

    pub fn new() -> Day12 {
        Day12 { ..Default::default() }
    }

}

impl AocDay for Day12 {

    fn day(&self) -> u8 {
        12
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
        let day12 = init(EX);
        assert_eq!(day12.part1(), "");
    }

    fn init(input: &str) -> Day12 {
        let mut day = Day12::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
