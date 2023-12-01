use crate::AocDay;

#[derive(Default)]
pub struct Day8 {
}

impl Day8 {

    pub fn new() -> Day8 {
        Day8 { ..Default::default() }
    }

}

impl AocDay for Day8 {

    fn day(&self) -> u8 {
        8
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
        let day8 = init(EX);
        assert_eq!(day8.part1(), "");
    }

    fn init(input: &str) -> Day8 {
        let mut day = Day8::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
