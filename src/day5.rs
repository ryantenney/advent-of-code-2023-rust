use std::collections::HashMap;
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day5 {
    seeds: Vec<u64>,
    mappings: HashMap<Section, Mapping>,
}

impl Day5 {

    pub fn new() -> Day5 {
        Day5 { ..Default::default() }
    }

    fn map(&self, seed: u64) -> u64 {
        let mut value = seed;
        for section in Section::SECTIONS.iter() {
            if let Some(mapping) = self.mappings.get(section) {
                value = mapping.map(value);
            } else {
                panic!("No mapping for section: {:?}", section);
            }
        }
        value
    }

}

impl AocDay for Day5 {

    fn info(&self) -> (u8, String) {
        (5, "If You Give A Seed A Fertilizer".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        let seeds_str = input[0].as_str();
        let seeds_start = seeds_str.find(':').unwrap() + 1;

        self.seeds = seeds_str[seeds_start..]
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut section: Option<Section> = None;
        for line in input.iter().skip(2).filter(|s| !s.is_empty()) {
            if section.is_none() || line.find(':').is_some() {
                let (section_str, _) = line.split_once(' ').unwrap();
                section = Section::by_text(section_str);
            } else if let Some(section) = section {
                self.mappings.entry(section)
                    .or_default()
                    .push(line.parse::<MappingRange>().unwrap());
            }
        }

        true
    }

    fn part1(&self) -> String {
        self.seeds.iter()
            .map(|seed| self.map(*seed))
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        self.seeds.iter().tuples()
            .flat_map(|(seed1, seed2)| *seed1..=(*seed1 + *seed2))
            .map(|seed| self.map(seed))
            .min()
            .unwrap()
            .to_string()
    }

}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Section {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Section {

    const SECTIONS: [Section; 7] = [
        Section::SeedToSoil,
        Section::SoilToFertilizer,
        Section::FertilizerToWater,
        Section::WaterToLight,
        Section::LightToTemperature,
        Section::TemperatureToHumidity,
        Section::HumidityToLocation,
    ];

    fn by_text(input: &str) -> Option<Section> {
        match input {
            "seed-to-soil" => Some(Section::SeedToSoil),
            "soil-to-fertilizer" => Some(Section::SoilToFertilizer),
            "fertilizer-to-water" => Some(Section::FertilizerToWater),
            "water-to-light" => Some(Section::WaterToLight),
            "light-to-temperature" => Some(Section::LightToTemperature),
            "temperature-to-humidity" => Some(Section::TemperatureToHumidity),
            "humidity-to-location" => Some(Section:: HumidityToLocation),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn new() -> Mapping {
        Mapping { ranges: Vec::new() }
    }

    fn push(&mut self, range: MappingRange) {
        self.ranges.push(range);
    }

    fn map(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.source_start <= value && value < range.source_start + range.length {
                return range.destination_start + (value - range.source_start);
            }
        }
        value
    }
}

#[derive(Clone, Debug)]
struct MappingRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl MappingRange {
    fn new(destination_start: u64, source_start: u64, length: u64) -> MappingRange {
        MappingRange { source_start, destination_start, length }
    }
}

impl FromStr for MappingRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<u64> = s.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();

        if let [destination_start, source_start, length] = split[..] {
            Ok(MappingRange::new(destination_start, source_start, length))
        } else {
            Err(anyhow!("Invalid mapping range: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "seeds: 79 14 55 13

                      seed-to-soil map:
                      50 98 2
                      52 50 48

                      soil-to-fertilizer map:
                      0 15 37
                      37 52 2
                      39 0 15

                      fertilizer-to-water map:
                      49 53 8
                      0 11 42
                      42 0 7
                      57 7 4

                      water-to-light map:
                      88 18 7
                      18 25 70

                      light-to-temperature map:
                      45 77 23
                      81 45 19
                      68 64 13

                      temperature-to-humidity map:
                      0 69 1
                      1 0 69

                      humidity-to-location map:
                      60 56 37
                      56 93 4";

    #[test]
    fn example() {
        let day5 = init(EX);
        assert_eq!(day5.part1(), "35");
        assert_eq!(day5.part2(), "46");
    }

    fn init(input: &str) -> Day5 {
        let mut day = Day5::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
