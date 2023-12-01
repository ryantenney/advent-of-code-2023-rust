use crate::AocDay;

#[derive(Default)]
pub struct Day22 {
}

impl Day22 {

    pub fn new() -> Day22 {
        Day22 { ..Default::default() }
    }

}

impl AocDay for Day22 {

    fn day(&self) -> u8 {
        22
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
        let day22 = init(EX);
        assert_eq!(day22.part1(), "");
    }

    fn init(input: &str) -> Day22 {
        let mut day = Day22::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
