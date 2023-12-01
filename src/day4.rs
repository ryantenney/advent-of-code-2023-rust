use crate::AocDay;

#[derive(Default)]
pub struct Day4 {
}

impl Day4 {

    pub fn new() -> Day4 {
        Day4 { ..Default::default() }
    }

}

impl AocDay for Day4 {

    fn day(&self) -> u8 {
        4
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
        let day4 = init(EX);
        assert_eq!(day4.part1(), "");
    }

    fn init(input: &str) -> Day4 {
        let mut day = Day4::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
