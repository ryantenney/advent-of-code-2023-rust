use crate::AocDay;

#[derive(Default)]
pub struct Day2 {
}

impl Day2 {

    pub fn new() -> Day2 {
        Day2 { ..Default::default() }
    }

}

impl AocDay for Day2 {

    fn day(&self) -> u8 {
        2
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
        let day2 = init(EX);
        assert_eq!(day2.part1(), "");
    }

    fn init(input: &str) -> Day2 {
        let mut day = Day2::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
