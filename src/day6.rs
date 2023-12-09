use crate::AocDay;

#[derive(Default)]
pub struct Day6 {
    races: Vec<Race>,
}

impl Day6 {

    pub fn new() -> Day6 {
        Day6 { ..Default::default() }
    }

}

impl AocDay for Day6 {

    fn info(&self) -> (u8, String) {
        (6, "Wait For It".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        assert_eq!(input.len(), 2);

        let times: Vec<_> = input[1]
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();

        let distances: Vec<_> = input[0]
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();

        self.races = times.iter()
            .zip(distances.iter())
            .map(|(distance, time)| Race::new(*time, *distance))
            .collect();

        true
    }

    fn part1(&self) -> String {
        self.races.iter()
            .map(|r| r.winning_options_quad())
            .product::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.races.clone().into_iter()
            .reduce(|a, b| a.append(b))
            .unwrap()
            .winning_options_quad()
            .to_string()
    }

}

#[derive(Clone, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Race { time, distance }
    }

    fn winning_options(&self) -> u64 {
        let mut count = 0;
        let time = self.time;
        let distance = self.distance;
        for release_time in 1..time {
            let speed = release_time;
            let remaining_time = time - release_time;
            let distance_travelled = speed * remaining_time;
            //println!("Hold the button for {release_time} milliseconds. After its remaining \
                     //{remaining_time} milliseconds of travel time, the boat will have gone \
                     //{distance_travelled} millimeters.");
            if distance_travelled > distance {
                count += 1;
            }
        }
        count
    }

    fn winning_options_quad(&self) -> u64 {
        let time = self.time;
        let distance = self.distance;
        let (x1, x2) = quad(1, time as i64, -(distance as i64));
        let x1 = x1.floor() as u64;
        let x2 = (x2.ceil() as u64).max(0);
        x1 - x2
    }

    fn append(&self, other: Self) -> Self {
        let time = (self.time.to_string() + &other.time.to_string())
            .parse::<u64>().unwrap();

        let distance = (self.distance.to_string() + &other.distance.to_string())
            .parse::<u64>().unwrap();

        Self { time, distance }
    }
}

fn quad(a: i64, b: i64, c: i64) -> (f64, f64) {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    let common = b.powi(2) - 4.0 * a * c;
    let a2 = 2.0 * a;
    let x1 = (-b + common.sqrt()) / a2;
    let x2 = (-b - common.sqrt()) / a2;
    (x1, x2)
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "Time:      7  15   30
                      Distance:  9  40  200";

    #[test]
    fn example() {
        let day6 = init(EX);
        assert_eq!(day6.part1(), "288");
        assert_eq!(day6.part2(), "71503");
    }

    fn init(input: &str) -> Day6 {
        let mut day = Day6::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
