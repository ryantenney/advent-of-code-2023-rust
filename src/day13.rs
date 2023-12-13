use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day13 {
    grids: Vec<Grid>,
}

impl Day13 {

    pub fn new() -> Day13 {
        Day13 { ..Default::default() }
    }

}

impl AocDay for Day13 {

    fn info(&self) -> (u8, String) {
        (13, "Point of Incidence".to_string())
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        let mut grids = Vec::new();
        let mut buf = Vec::new();
        for line in input {
            if line.is_empty() {
                grids.push(Grid::new(buf.clone()));
                buf.clear();
            } else {
                buf.push(line.chars().collect());
            }
        }

        if !buf.is_empty() {
            grids.push(Grid::new(buf.clone()));
        }

        self.grids = grids;

        true
    }

    fn part1(&self) -> String {
        let mut result = 0;
        for grid in self.grids.iter() {
            if let Some(val) = grid.find_reflect_vert() {
                result += val;
            }
            if let Some(val) = grid.find_reflect_horiz() {
                result += 100 * val;
            }
        }
        result.to_string()
    }

    fn part2(&self) -> String {
        let mut result = 0;
        for grid in self.grids.iter() {
            if let Some(val) = grid.find_reflect_with_smudge_vert() {
                result += val;
            }
            if let Some(val) = grid.find_reflect_with_smudge_horiz() {
                result += 100 * val;
            }
        }
        result.to_string()
    }

}

#[derive(Debug, Clone)]
struct Grid {
    horiz: Vec<u32>,
    vert: Vec<u32>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let horiz = grid.iter().map(|row| to_u32(row)).collect();
        let vert = transpose(&grid).iter().map(|row| to_u32(row)).collect();
        Grid { horiz, vert }
    }

    fn find_reflect_vert(&self) -> Option<u32> {
        find_reflect(&self.vert)
    }

    fn find_reflect_horiz(&self) -> Option<u32> {
        find_reflect(&self.horiz)
    }

    fn find_reflect_with_smudge_vert(&self) -> Option<u32> {
        find_reflect_with_smudge(&self.vert)
    }

    fn find_reflect_with_smudge_horiz(&self) -> Option<u32> {
        find_reflect_with_smudge(&self.horiz)
    }
}

fn find_reflect(vec: &Vec<u32>) -> Option<u32> {
    let len = vec.len();
    for i in 1..len {
        if vec.iter().rev().skip(len - i).zip(vec.iter().skip(i)).all(|(a, b)| a == b) {
            return Some(i as u32);
        }
    }
    None
}

fn find_reflect_with_smudge(vec: &Vec<u32>) -> Option<u32> {
    let len = vec.len();
    'outer: for i in 1..len {
        let mut smudge = false;
        for (a, b) in vec.iter().rev().skip(len - i).zip(vec.iter().skip(i)) {
            if a == b {
                continue;
            }

            let diff = a ^ b;
            if smudge || diff & (diff - 1) != 0 {
                continue 'outer;
            }

            smudge = true;
        }
        if smudge {
            return Some(i as u32);
        }
    }
    None
}

fn to_u32(str: &Vec<char>) -> u32 {
    let mut val = 0;
    for c in str {
        val <<= 1;
        if *c == '#' {
            val |= 1;
        }
    }
    val
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();
    for i in 0..grid[0].len() {
        let mut row = Vec::new();
        for j in 0..grid.len() {
            row.push(grid[j][i]);
        }
        new_grid.push(row);
    }
    new_grid
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "#.##..##.
                       ..#.##.#.
                       ##......#
                       ##......#
                       ..#.##.#.
                       ..##..##.
                       #.#.##.#.

                       #...##..#
                       #....#..#
                       ..##..###
                       #####.##.
                       #####.##.
                       ..##..###
                       #....#..#";

    const EX2: &str = "#..##..#######..#
                       .##..##.#.##....#
                       ##.##.##..##...##
                       .#....#..#.###.##
                       ..####...#...##.#
                       ###...####...#..#
                       ########...##..##
                       #......##....#.##
                       ..#..#...###...#.
                       #.####.#.#..#.#.#
                       #.####.#.#..#.#.#";

    #[test]
    fn example() {
        let day13 = init(EX1);
        assert_eq!(day13.part1(), "405");
        assert_eq!(day13.part2(), "400");

        let day13 = init(EX2);
        assert_eq!(day13.part1(), "1000");
    }

    fn init(input: &str) -> Day13 {
        let mut day = Day13::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
