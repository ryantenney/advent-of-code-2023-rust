use crate::AocDay;

#[derive(Default)]
pub struct Day14 {
}

impl Day14 {

    pub fn new() -> Day14 {
        Day14 { ..Default::default() }
    }

}

impl AocDay for Day14 {

    fn day(&self) -> u8 {
        14
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
        let day14 = init(EX);
        assert_eq!(day14.part1(), "");
    }

    fn init(input: &str) -> Day14 {
        let mut day = Day14::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
