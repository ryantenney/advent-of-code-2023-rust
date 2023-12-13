use crate::aocday::AocDay;
use crate::{day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21, day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9};

pub fn build_days() -> Vec<Box<dyn AocDay>> {
    vec![
        Box::new(day1::Day1::new()),
        Box::new(day2::Day2::new()),
        Box::new(day3::Day3::new()),
        Box::new(day4::Day4::new()),
        Box::new(day5::Day5::new()),
        Box::new(day6::Day6::new()),
        Box::new(day7::Day7::new()),
        Box::new(day8::Day8::new()),
        Box::new(day9::Day9::new()),
        Box::new(day10::Day10::new()),
        Box::new(day11::Day11::new()),
        Box::new(day12::Day12::new()),
        Box::new(day13::Day13::new()),
        Box::new(day14::Day14::new()),
        Box::new(day15::Day15::new()),
        Box::new(day16::Day16::new()),
        Box::new(day17::Day17::new()),
        Box::new(day18::Day18::new()),
        Box::new(day19::Day19::new()),
        Box::new(day20::Day20::new()),
        Box::new(day21::Day21::new()),
        Box::new(day22::Day22::new()),
        Box::new(day23::Day23::new()),
        Box::new(day24::Day24::new()),
        Box::new(day25::Day25::new()),
    ]
}
