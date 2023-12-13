pub trait AocDay {

    fn day(&self) -> u8 {
        self.info().0
    }

    fn info(&self) -> (u8, String) {
        (0, String::new())
    }

    fn init(&mut self, _input: &Vec<String>) -> bool {
        false
    }

    fn part1(&self) -> String;

    fn part2(&self) -> String;

}
