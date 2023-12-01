use crate::AocDay;

#[derive(Default)]
pub struct Day21 {
}

impl Day21 {

    pub fn new() -> Day21 {
        Day21 { ..Default::default() }
    }

}

impl AocDay for Day21 {

    fn day(&self) -> u8 {
        21
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
        let day21 = init(EX);
        assert_eq!(day21.part1(), "");
    }

    fn init(input: &str) -> Day21 {
        let mut day = Day21::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
