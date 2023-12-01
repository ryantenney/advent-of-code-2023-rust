use crate::AocDay;

#[derive(Default)]
pub struct Day20 {
}

impl Day20 {

    pub fn new() -> Day20 {
        Day20 { ..Default::default() }
    }

}

impl AocDay for Day20 {

    fn day(&self) -> u8 {
        20
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
        let day20 = init(EX);
        assert_eq!(day20.part1(), "");
    }

    fn init(input: &str) -> Day20 {
        let mut day = Day20::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
