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
mod timer;
mod util;
mod aocday;
mod days;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use std::time::{Duration, Instant};
use anyhow::anyhow;
use reqwest::blocking::Client;
use aocday::AocDay;
use days::build_days;
use RunMode::Unlocked;
use timer::Timer;
use util::{end_day, read_input};
use crate::RunMode::{All, Single, Today};

const YEAR: i32 = 2023;
const RUN_DAY: Option<u8> = None;
const RUN_MODE: RunMode = Today;

#[derive(Clone, Copy, Debug, PartialEq)]
enum RunMode {
    All,
    Today,
    Single,
    Unlocked,
}

fn main() {
    let today = end_day(YEAR);
    let mut days = build_days();

    let mut timer = Timer::new();
    for day in days.iter_mut() {
        if RUN_MODE == Single {
            if let Some(run_day) = RUN_DAY {
                if run_day != day.day() {
                    continue;
                }
            }
        } else if RUN_MODE != All {
            if let Some(today) = today {
                if (RUN_MODE == Today && day.day() != today) ||
                    (RUN_MODE == Unlocked && day.day() > today) {
                    continue;
                }
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

    println!("Total: {:?}", timer.duration);
}

fn print_part(part_number: u8, solution: String, duration: Duration) {
    if solution.contains('\n') {
        println!("  Part {}: ({:?})\n    {}", part_number, duration, solution.replace('\n', "\n    "));
    } else {
        println!("  Part {}: {} ({:?})", part_number, solution, duration);
    }
}
