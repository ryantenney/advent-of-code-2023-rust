use crate::AocDay;

#[derive(Default)]
pub struct Day25 {
}

impl Day25 {

    pub fn new() -> Day25 {
        Day25 { ..Default::default() }
    }

}

impl AocDay for Day25 {

    fn day(&self) -> u8 {
        25
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
        let day25 = init(EX);
        assert_eq!(day25.part1(), "");
    }

    fn init(input: &str) -> Day25 {
        let mut day = Day25::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
