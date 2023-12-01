use crate::AocDay;

#[derive(Default)]
pub struct Day23 {
}

impl Day23 {

    pub fn new() -> Day23 {
        Day23 { ..Default::default() }
    }

}

impl AocDay for Day23 {

    fn day(&self) -> u8 {
        23
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
        let day23 = init(EX);
        assert_eq!(day23.part1(), "");
    }

    fn init(input: &str) -> Day23 {
        let mut day = Day23::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
