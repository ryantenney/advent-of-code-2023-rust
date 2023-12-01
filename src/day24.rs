use crate::AocDay;

#[derive(Default)]
pub struct Day24 {
}

impl Day24 {

    pub fn new() -> Day24 {
        Day24 { ..Default::default() }
    }

}

impl AocDay for Day24 {

    fn day(&self) -> u8 {
        24
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
        let day24 = init(EX);
        assert_eq!(day24.part1(), "");
    }

    fn init(input: &str) -> Day24 {
        let mut day = Day24::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
