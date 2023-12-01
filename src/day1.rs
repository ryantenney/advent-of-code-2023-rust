use crate::AocDay;

#[derive(Default)]
pub struct Day1 {
    input: Vec<String>
}

impl Day1 {

    pub fn new() -> Day1 {
        Day1 { ..Default::default() }
    }

}

impl AocDay for Day1 {

    fn day(&self) -> u8 {
        1
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.input = input.clone();
        true
    }

    fn part1(&self) -> String {
        self.input.iter().map(|s| {
            let digits: Vec<u32> = s.chars()
                .map(|c| c.to_digit(10))
                .filter(|d| d.is_some())
                .map(|d| d.unwrap()).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        }).sum::<u32>().to_string()
    }

    fn part2(&self) -> String {
        self.input.iter()
            .map(|s| parse_str(s))
            .sum::<u32>()
            .to_string()
    }

}

fn parse_str(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let len = input.len();
    let mut first = 0;
    let mut last = 0;

    while index < len {
        let value = match chars[index] {
            '1'..='9' => chars[index].to_digit(10).unwrap(),
            'o' => eq(&chars, "one", index, 1),
            't' => match safe_index(&chars, index + 1) {
                'w' => eq(&chars, "two", index, 2),
                'h' => eq(&chars, "three", index, 3),
                _ => 0,
            },
            'f' => match safe_index(&chars, index + 1) {
                'o' => eq(&chars, "four", index, 4),
                'i' => eq(&chars, "five", index, 5),
                _ => 0,
            },
            's' => match safe_index(&chars, index + 1) {
                'i' => eq(&chars, "six", index, 6),
                'e' => eq(&chars, "seven", index, 7),
                _ => 0,
            },
            'e' => eq(&chars, "eight", index, 8),
            'n' => eq(&chars, "nine", index, 9),
            _ => 0,
        };

        if value != 0 {
            if first == 0 {
                first = value;
                last = value;
            } else {
                last = value;
            }
        }

        index += 1;
    }

    first * 10 + last
}

fn safe_index(chars: &Vec<char>, index: usize) -> char {
    if index >= chars.len() {
        ' '
    } else {
        chars[index]
    }
}

fn eq(chars: &Vec<char>, string: &str, index: usize, value: u32) -> u32 {
    let len = string.len();
    let expected: Vec<char> = string.chars().collect();
    if index + len > chars.len() {
        return 0
    }

    for x in 0..len {
        if chars[index + x] != expected[x] {
            return 0
        }
    }

    value
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "1abc2
                       pqr3stu8vwx
                       a1b2c3d4e5f
                       treb7uchet";

    const EX2: &str = "two1nine
                       eightwothree
                       abcone2threexyz
                       xtwone3four
                       4nineeightseven2
                       zoneight234
                       7pqrstsixteen";

    #[test]
    fn example1() {
        let day = init(EX1);
        assert_eq!(day.part1(), "142");
    }

    #[test]
    fn example2() {
        let day = init(EX2);
        day.input.iter().for_each(|s| println!("{} == {}", s, parse_str(s)));
        assert_eq!(day.part2(), "281");
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str("oneight"), 18);
        assert_eq!(parse_str("6l"), 66);
    }

    fn init(input: &str) -> Day1 {
        let mut day = Day1::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
