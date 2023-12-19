use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::str::FromStr;
use anyhow::anyhow;
use itertools::Itertools;
use crate::aocday::AocDay;

#[derive(Default)]
pub struct Day19 {
    states: Vec<Xmas>,
    workflows: HashMap<String, Workflow>,
}

impl Day19 {

    pub fn new() -> Day19 {
        Day19 { ..Default::default() }
    }

}

impl AocDay for Day19 {

    fn day(&self) -> u8 {
        19
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        let (states, workflows): (Vec<_>, Vec<_>) = input.iter()
            .filter(|s| !s.is_empty())
            .partition(|s| s.starts_with('{'));

        self.states = states.iter()
            .map(|s| s.parse::<Xmas>())
            .flatten()
            .collect();

        self.workflows = workflows.iter()
            .map(|s| s.parse::<Workflow>())
            .flatten()
            .map(|w| (w.id.clone(), w.clone()))
            .collect();

        true
    }

    fn part1(&self) -> String {
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();
        for state in self.states.iter() {
            let destination = state.process(&self.workflows);
            match destination {
                Destination::Accept => accepted.push(state),
                Destination::Reject => rejected.push(state),
                _ => panic!("Invalid destination"),
            }
        }
        accepted.iter().map(|s| s.sum()).sum::<u32>().to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }

}

#[derive(Clone, Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
    fallthrough: Destination,
}

impl Workflow {
    fn process(&self, state: &Xmas) -> Destination {
        for rule in self.rules.iter() {
            if let Some(destination) = rule.apply(state) {
                return destination.clone();
            }
        }
        self.fallthrough.clone()
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{},{}}}",
               self.id,
               self.rules.iter().map(|r| r.to_string()).join(","),
               self.fallthrough)
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflow_id, rules_str) = s.strip_suffix("}").unwrap().split_once('{').unwrap();

        let fallthrough_index = rules_str.rfind(',')
            .ok_or(anyhow!("Invalid workflow rules, expected a fallthrough"))?;

        let fallthrough = Destination::from_str(&rules_str[fallthrough_index + 1..])?;

        let rules = rules_str[..fallthrough_index]
            .split(',')
            .map(|s| s.parse::<Rule>())
            .flatten()
            .collect_vec();

        Ok(Workflow {
            id: workflow_id.to_string(),
            rules,
            fallthrough,
        })
    }
}

#[derive(Clone, Debug)]
struct Rule {
    param_name: Param,
    operator: Operator,
    threshold: u32,
    destination: Destination,
}

impl Rule {
    fn apply(&self, state: &Xmas) -> Option<Destination> {
        let param_value = state.get(self.param_name);
        match self.operator {
            Operator::GreaterThan => {
                if param_value > self.threshold {
                    Some(self.destination.clone())
                } else {
                    None
                }
            }
            Operator::LessThan => {
                if param_value < self.threshold {
                    Some(self.destination.clone())
                } else {
                    None
                }
            }
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}:{}",
               self.param_name,
               self.operator,
               self.threshold,
               self.destination)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((expression, destination)) = s.split_once(':') {
            Ok(Rule {
                param_name: Param::from(expression.chars().nth(0).unwrap()),
                operator: Operator::from(expression.chars().nth(1).unwrap()),
                threshold: expression[2..].parse()?,
                destination: Destination::from_str(destination)?,
            })
        } else {
            Err(anyhow!("Invalid rule"))
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Param {
    X,
    M,
    A,
    S,
}

impl Display for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::X => write!(f, "x"),
            Param::M => write!(f, "m"),
            Param::A => write!(f, "a"),
            Param::S => write!(f, "s"),
        }
    }
}

impl From<char> for Param {
    fn from(c: char) -> Self {
        match c {
            'x' => Param::X,
            'm' => Param::M,
            'a' => Param::A,
            's' => Param::S,
            _ => panic!("Invalid param"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    GreaterThan,
    LessThan,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThan => write!(f, "<"),
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '>' => Operator::GreaterThan,
            '<' => Operator::LessThan,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Clone, Debug)]
enum Destination {
    Workflow(String),
    Accept,
    Reject
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Workflow(id) => write!(f, "{}", id),
            Destination::Accept => write!(f, "A"),
            Destination::Reject => write!(f, "R"),
        }
    }
}

impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(anyhow!("Invalid destination")),
            "A" => Ok(Destination::Accept),
            "R" => Ok(Destination::Reject),
            _ => Ok(Destination::Workflow(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Xmas {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Xmas {
    fn get(&self, param: Param) -> u32 {
        match param {
            Param::X => self.x,
            Param::M => self.m,
            Param::A => self.a,
            Param::S => self.s,
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn process(&self, workflows: &HashMap<String, Workflow>) -> Destination {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match workflow.process(self) {
                Destination::Workflow(id) => {
                    workflow = workflows.get(&id).unwrap();
                }
                dest => return dest
            }
        }
    }
}

impl Display for Xmas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}

impl FromStr for Xmas {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = s
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect_tuple()
            .unwrap();

        Ok(Xmas {
            x: x.strip_prefix("x=").unwrap().parse()?,
            m: m.strip_prefix("m=").unwrap().parse()?,
            a: a.strip_prefix("a=").unwrap().parse()?,
            s: s.strip_prefix("s=").unwrap().parse()?,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn example() {
        let day19 = init(EX);
        assert_eq!(day19.part1(), "19114");
        assert_eq!(day19.part2(), "");
    }

    fn init(input: &str) -> Day19 {
        let mut day = Day19::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
