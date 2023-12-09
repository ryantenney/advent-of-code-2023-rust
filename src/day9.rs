use std::collections::VecDeque;
use std::str::FromStr;
use crate::AocDay;

#[derive(Default)]
pub struct Day9 {
    layers: Vec<Layers>,
}

impl Day9 {

    pub fn new() -> Day9 {
        Day9 { ..Default::default() }
    }

}

impl AocDay for Day9 {

    fn day(&self) -> u8 {
        9
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.layers = input.iter().map(|s| s.parse::<Layers>().unwrap()).collect();

        true
    }

    fn part1(&self) -> String {
        let mut layers = self.layers.iter().map(|l| l.copy()).collect::<Vec<_>>();
        for layer in layers.iter_mut() {
            while !layer.consistent() {
                layer.derive_layer();
            }
            layer.extend();
        }
        layers.iter().map(|l| l.values.first().unwrap().back().unwrap()).sum::<i32>().to_string()
    }

    fn part2(&self) -> String {
        let mut layers = self.layers.iter().map(|l| l.copy()).collect::<Vec<_>>();
        for layer in layers.iter_mut() {
            while !layer.consistent() {
                layer.derive_layer();
            }
            layer.extend_front();
        }
        layers.iter().map(|l| l.values.first().unwrap().front().unwrap()).sum::<i32>().to_string()
    }

}

#[derive(Debug)]
struct Layers {
    values: Vec<VecDeque<i32>>,
}

impl Layers {

    fn new() -> Layers {
        Layers { values: Vec::new() }
    }

    fn copy(&self) -> Layers {
        let mut layers = Layers::new();
        for layer in &self.values {
            layers.add(layer.clone());
        }
        layers
    }

    fn derive_layer(&mut self) {
        let mut result: VecDeque<i32> = VecDeque::new();
        let last = self.values.last().unwrap();
        let mut value = last.front().unwrap();
        for next_value in last.iter().skip(1) {
            result.push_back(next_value - value);
            value = next_value;
        }
        self.add(result);
    }

    fn add(&mut self, layer: VecDeque<i32>) {
        self.values.push(layer);
    }

    fn consistent(&self) -> bool {
        let last = self.values.last().expect("No layers");
        let mut value = last.front().unwrap();
        for next_value in last.iter().skip(1) {
            if value != next_value {
                return false;
            }
            value = next_value;
        }
        true
    }

    fn extend(&mut self) {
        let mut last: Option<i32> = None;
        for layer in self.values.iter_mut().rev() {
            let i = layer.back().unwrap().clone();
            if let Some(l) = last {
                last = Some(l + i);
                layer.push_back(l + i);
            } else {
                last = Some(i);
                layer.push_back(i);
            }
        }
    }

    fn extend_front(&mut self) {
        let mut last: Option<i32> = None;
        for layer in self.values.iter_mut().rev() {
            let i = layer.front().unwrap().clone();
            if let Some(l) = last {
                last = Some(i - l);
                layer.push_front(i - l);
            } else {
                last = Some(i);
                layer.push_front(i);
            }
        }
    }

}

impl FromStr for Layers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut layers = Layers::new();
        let layer = s.split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        layers.add(layer);
        Ok(layers)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "0 3 6 9 12 15
                      1 3 6 10 15 21
                      10 13 16 21 30 45";

    #[test]
    fn example() {
        let day9 = init(EX);
        assert_eq!(day9.part1(), "114");
        assert_eq!(day9.part2(), "2");
    }

    fn init(input: &str) -> Day9 {
        let mut day = Day9::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
