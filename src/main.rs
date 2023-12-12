#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use std::time::{Duration, Instant};
use anyhow::anyhow;
use datetime::convenience::Today;
use datetime::{DatePiece, LocalDate, Month};
use reqwest::blocking::Client;

const RUN_DAY: Option<u8> = None;

trait AocDay {
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

fn read_lines(name: String) -> Result<Vec<String>, std::io::Error> {
    let mut path = env::current_dir().unwrap();
    path.set_file_name(name);

    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(str::to_string).collect())
}

fn read_session_token() -> Result<String, anyhow::Error> {
    let mut path = env::current_dir()?;
    path.push(".session_token");

    let contents = fs::read_to_string(path)?;
    Ok(contents.trim().to_string())
}

fn read_input(day: u8) -> Result<Vec<String>, anyhow::Error> {
    let mut path = env::current_dir()?;
    path.push(format!("src/day{day}.txt"));

    if !path.exists() {
        return download_input(day, path);
    }

    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(str::to_string).collect())
}

fn download_input<P: AsRef<Path>>(day: u8, path: P) -> Result<Vec<String>, anyhow::Error> {
    let session_token = read_session_token()?;
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let input = Client::new().get(&url)
        .header("Cookie", format!("session={session_token}"))
        .header("User-Agent", "github.com/abc/advent-of-code-2023 by abc@xyz.com")
        .send()
        .unwrap()
        .text()?;

    fs::write(path, &input)?;

    Ok(input.lines().map(str::to_string).collect())
}

fn main() {
    let end_day = end_day();

    let mut days: Vec<Box<dyn AocDay>> = Vec::new();
    days.push(Box::new(day1::Day1::new()));
    days.push(Box::new(day2::Day2::new()));
    days.push(Box::new(day3::Day3::new()));
    days.push(Box::new(day4::Day4::new()));
    days.push(Box::new(day5::Day5::new()));
    days.push(Box::new(day6::Day6::new()));
    days.push(Box::new(day7::Day7::new()));
    days.push(Box::new(day8::Day8::new()));
    days.push(Box::new(day9::Day9::new()));
    days.push(Box::new(day10::Day10::new()));
    days.push(Box::new(day11::Day11::new()));
    days.push(Box::new(day12::Day12::new()));
    days.push(Box::new(day13::Day13::new()));
    days.push(Box::new(day14::Day14::new()));
    days.push(Box::new(day15::Day15::new()));
    days.push(Box::new(day16::Day16::new()));
    days.push(Box::new(day17::Day17::new()));
    days.push(Box::new(day18::Day18::new()));
    days.push(Box::new(day19::Day19::new()));
    days.push(Box::new(day20::Day20::new()));
    days.push(Box::new(day21::Day21::new()));
    days.push(Box::new(day22::Day22::new()));
    days.push(Box::new(day23::Day23::new()));
    days.push(Box::new(day24::Day24::new()));
    days.push(Box::new(day25::Day25::new()));

    let mut timer = Timer::new();
    for day in days.iter_mut() {
        if let Some(run_day) = RUN_DAY {
            if run_day != day.day() {
                continue;
            }
        } else if let Some(end_day) = end_day {
            if end_day < day.day() {
                continue;
            }
        }

        let input = read_input(day.day());
        if input.is_err() {
            println!("Day {}: input not found", day.day());


            continue;
        }

        let (init_success, init_duration) = timer.time_with_return(|| day.init(&input.unwrap()));
        if !init_success {
            println!("Day {}: not implemented", day.day());
            continue
        }

        let (day_number, day_name) = day.info();
        if day_name.is_empty() {
            println!("Day {}", day_number);
        } else {
            println!("Day {}: {}", day_number, day_name);
        }

        println!("  Init  : ({:?})", init_duration);

        let (part1, part1_duration) = timer.time_with_return(|| day.part1());
        print_part(1, part1, part1_duration);

        let (part2, part2_duration) = timer.time_with_return(|| day.part2());
        print_part(2, part2, part2_duration);
    }

    println!("Total: {:?}", timer.0);
}

fn print_part(part_number: u8, solution: String, duration: Duration) {
    if solution.contains('\n') {
        println!("  Part {}: ({:?})\n    {}", part_number, duration, solution.replace('\n', "\n    "));
    } else {
        println!("  Part {}: {} ({:?})", part_number, solution, duration);
    }
}

fn end_day() -> Option<u8> {
    let date = LocalDate::today();
    if date.year() == 2023 && date.month() == Month::December {
        Some(date.day() as u8)
    } else {
        None
    }
}

struct Timer(Duration);

impl Timer {
    fn new() -> Timer {
        Timer(Duration::ZERO)
    }

    fn time(&mut self, f: impl FnOnce()) -> Duration {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        self.0 += elapsed;
        elapsed
    }

    fn time_with_return<T>(&mut self, f: impl FnOnce() -> T) -> (T, Duration) {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        self.0 += elapsed;
        (result, elapsed)
    }

    fn time_with_result<T, E>(&mut self, f: impl FnOnce() -> Result<T, E>) -> Result<(T, Duration), E> {
        let start = Instant::now();
        let result = f()?;
        let elapsed = start.elapsed();
        self.0 += elapsed;
        Ok((result, elapsed))
    }

}
