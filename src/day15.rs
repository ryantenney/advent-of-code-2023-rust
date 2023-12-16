use std::cell::Cell;
use std::fmt::{Display, Formatter};
use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day15 {
    iv: Vec<String>,
}

impl Day15 {

    pub fn new() -> Day15 {
        Day15 { ..Default::default() }
    }

}

impl AocDay for Day15 {

    fn info(&self) -> (u8, String) {
        (15, "".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.iv = input[0].split(',').map(|s| s.trim().to_string()).collect();

        true
    }

    fn part1(&self) -> String {
        self.iv
            .iter()
            .map(|s| hash(s))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

        for lens_str in self.iv.iter() {
            if let Some((id_str, value_str)) = lens_str.split_once('=') {
                let hash = hash(id_str);
                let value = value_str.parse::<u32>().unwrap();
                let lens = Lens::new(id_str.to_string(), value);
                let mut edited = false;
                for mut lens in boxes.get_mut(hash).unwrap().iter_mut() {
                    if lens.id == id_str {
                        lens.focal_length = value;
                        edited = true;
                        break;
                    }
                }
                if !edited {
                    boxes.get_mut(hash).unwrap().push(lens);
                }
            } else if let Some(id_str) = lens_str.strip_suffix('-') {
                let hash = hash(id_str);
                boxes.get_mut(hash).unwrap().retain(|lens| lens.id != id_str);
            }
        }

        boxes.iter()
            .enumerate()
            .flat_map(|(box_index, lenses)| {
                lenses.iter()
                    .enumerate()
                    .map(move |(slot_index, lens)| (box_index + 1) * (slot_index + 1) * lens.focal_length as usize)
            })
            .sum::<usize>()
            .to_string()
    }

}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h = ((h + c as usize) * 17) % 256;
    }
    h
}

#[derive(Default)]
struct Lens {
    id: String,
    focal_length: u32,
}

impl Lens {
    fn new(id: String, focal_length: u32) -> Lens {
        Lens { id, focal_length }
    }
}

impl Display for Lens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.id, self.focal_length)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn example() {
        let day15 = init(EX);
        assert_eq!(day15.part1(), "1320");
        assert_eq!(day15.part2(), "145");
    }

    fn init(input: &str) -> Day15 {
        let mut day = Day15::new();
        day.init(&input.split('\n').map(|s| s.to_string()).collect());
        day
    }

}
