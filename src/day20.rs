use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, write};
use std::str::FromStr;
use itertools::Itertools;
use crate::aocday::AocDay;
use crate::day20::Signal::{High, Low};
use crate::math::gcd;

#[derive(Default)]
pub struct Day20 {
    circuit: Circuit,
}

impl Day20 {

    pub fn new() -> Day20 {
        Day20 { ..Default::default() }
    }

}

impl AocDay for Day20 {

    fn day(&self) -> u8 {
        20
    }

    fn init(&mut self, input: &Vec<String>) -> bool {
        self.circuit.init(input.iter()
            .map(|s| Module::from_str(s))
            .flatten()
            .collect());

        true
    }

    fn part1(&self) -> String {
        let mut circuit = self.circuit.clone();

        for _ in 0..1000 {
            circuit.button();
        }

        let (low, high) = circuit.sum();

        (low * high).to_string()
    }

    fn part2(&self) -> String {
        let mut circuit = self.circuit.clone();

        let conj = circuit.module("rx").unwrap()
            .inputs
            .keys()
            .exactly_one()
            .unwrap();

        let mut conj_inputs_id: HashMap<String, u64> = circuit.module(conj).unwrap()
            .inputs
            .keys()
            .map(|k| (k.clone(), 0))
            .collect();

        let mut presses: u64 = 0;
        let mut output = 0;
        while conj_inputs_id.values().any(|c| *c == 0) {
            presses += 1;
            circuit.button();

            for (input_id, value) in conj_inputs_id.iter_mut() {
                if *value == 0 && circuit.module(&input_id).unwrap().low_pulses == 1 {
                    *value = presses;
                    if output == 0 {
                        output = presses;
                    } else {
                        // lcm
                        output = output * presses / gcd(output, presses);
                    }
                }
            }
        }

        output.to_string()
    }
}

#[derive(Default, Clone)]
struct Circuit {
    pulses: VecDeque<Pulse>,
    modules: HashMap<String, Module>,
}

impl Circuit {
    fn init(&mut self, modules: Vec<Module>) {
        let mut modules: HashMap<_, _> = modules.iter()
            .map(|module| (module.id.clone(), module.clone()))
            .collect();

        modules.insert("rx".to_string(), Module::new("rx".to_string(), ModuleType::None, vec![]));
        modules.insert("output".to_string(), Module::new("output".to_string(), ModuleType::None, vec![]));

        let input_map = modules.values()
            .flat_map(|v| v.output_ids.iter()
                .map(|output_id| (output_id.clone(), v.id.clone())))
            .into_group_map();

        for (module_id, input_ids) in input_map {
            if let Some(module) = modules.get_mut(module_id.as_str()) {
                module.set_inputs(input_ids.clone());
            }
        }

        self.modules = modules;
    }

    fn button(&mut self) {
        self.pulses.push_back(Pulse::new("button", "broadcaster", Low));
        while let Some(pulse) = self.pulses.pop_front() {
            self.handle_pulse(pulse);
        }
    }

    fn handle_pulse(&mut self, pulse: Pulse) {
        if let Some(module) = self.modules.get_mut(&pulse.destination) {
            self.pulses.extend(module.handle_pulse(pulse));
        }
    }

    fn module(&self, id: &str) -> Option<&Module> {
        self.modules.get(id)
    }

    fn sum(&self) -> (usize, usize) {
        self.modules.iter()
            .fold((0, 0), |(low, high), (_, module)| (low + module.low_pulses, high + module.high_pulses))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pulse {
    source: String,
    destination: String,
    signal: Signal,
}

impl Pulse {
    fn new(source: &str, destination: &str, signal: Signal) -> Pulse {
        Pulse {
            source: source.to_string(),
            destination: destination.to_string(),
            signal,
        }
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.source, self.signal, self.destination)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Signal {
    Low,
    High,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Low => write!(f, "-low->"),
            High => write!(f, "-high->"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ModuleType {
    None,
    FlipFlop,
    Conjunction
}

impl Display for ModuleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleType::FlipFlop => write!(f, "%"),
            ModuleType::Conjunction => write!(f, "&"),
            _ => Ok(()),
        }
    }
}

impl TryFrom<char> for ModuleType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '%' => Ok(ModuleType::FlipFlop),
            '&' => Ok(ModuleType::Conjunction),
            _ => Err(anyhow::anyhow!("Invalid module type: {}", value)),
        }
    }
}

#[derive(Clone)]
struct Module {
    id: String,
    module_type: ModuleType,
    output_ids: Vec<String>,
    flip_flop_state: bool,
    inputs: HashMap<String, Signal>,
    low_pulses: usize,
    high_pulses: usize,
}

impl Module {
    fn new(id: String, module_type: ModuleType, output_ids: Vec<String>) -> Self {
        Module {
            id,
            module_type,
            output_ids,
            flip_flop_state: false,
            inputs: HashMap::new(),
            low_pulses: 0,
            high_pulses: 0,
        }
    }

    fn output(id: String) -> Self {
        Self::new(id, ModuleType::None, vec![])
    }

    fn pulse(&mut self) {
        self.low_pulses += 1;
    }

    fn set_inputs(&mut self, inputs: Vec<String>) {
        self.inputs = inputs.iter()
            .map(|input_id| (input_id.clone(), Low))
            .collect();
    }

    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match pulse.signal {
            Low => self.low_pulses += 1,
            High => self.high_pulses += 1,
        }

        match self.module_type {
            ModuleType::None => {
                self.output_pulse(pulse.signal)
            },
            ModuleType::FlipFlop => {
                if pulse.signal == Low {
                    self.flip_flop_state = !self.flip_flop_state;
                    if self.flip_flop_state {
                        self.output_pulse(High)
                    } else {
                        self.output_pulse(Low)
                    }
                } else {
                    vec![]
                }
            },
            ModuleType::Conjunction => {
                self.inputs.insert(pulse.source, pulse.signal);
                if self.inputs.values().all(|v| *v == High) {
                    self.output_pulse(Low)
                } else {
                    self.output_pulse(High)
                }
            }
        }
    }

    fn output_pulse(&self, signal: Signal) -> Vec<Pulse> {
        self.output_ids.iter()
            .map(|id| Pulse::new(&self.id, id, signal))
            .collect()
    }

}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} -> {}", self.module_type, self.id, self.output_ids.iter().join(", "))
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (module_spec, outputs) = s.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        let (module_type, module_id) = if let Ok(module_type) = ModuleType::try_from(module_spec.chars().nth(0).unwrap()) {
            (module_type, module_spec[1..].to_string())
        } else {
            (ModuleType::None, module_spec.to_string())
        };

        Ok(Module::new(module_id, module_type, outputs))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "broadcaster -> a, b, c
                       %a -> b
                       %b -> c
                       %c -> inv
                       &inv -> a";

    const EX2: &str = "broadcaster -> a
                       %a -> inv, con
                       &inv -> b
                       %b -> con
                       &con -> output";

    #[test]
    fn example() {
        let day20 = init(EX1);
        assert_eq!(day20.part1(), "32000000");

        let day20 = init(EX2);
        assert_eq!(day20.part1(), "11687500");
    }

    fn init(input: &str) -> Day20 {
        let mut day = Day20::new();
        day.init(&input.split('\n').map(|s| s.trim().to_string()).collect());
        day
    }

}
