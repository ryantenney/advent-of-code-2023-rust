use crate::AocDay;

#[derive(Default)]
pub struct Day17 {
}

impl Day17 {

    pub fn new() -> Day17 {
        Day17 { ..Default::default() }
    }

}

impl AocDay for Day17 {

    fn day(&self) -> u8 {
        17
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
        let day17 = init(EX);
        assert_eq!(day17.part1(), "");
    }

    fn init(input: &str) -> Day17 {
        let mut day = Day17::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
