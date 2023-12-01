use crate::AocDay;

#[derive(Default)]
pub struct Day9 {
}

impl Day9 {

    pub fn new() -> Day9 {
        Day9 { ..Default::default() }
    }

}

impl AocDay for Day9 {

    fn day(&self) -> u8 {
        9
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
        let day9 = init(EX);
        assert_eq!(day9.part1(), "");
    }

    fn init(input: &str) -> Day9 {
        let mut day = Day9::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
