use std::collections::HashMap;
use regex::Regex;
use super::{into_next, Instruction};

#[derive(Default)]
pub struct Stv {
    next: Option<Box<dyn Instruction>>,
}

impl Stv {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Stv {
    fn handle(&mut self, stack: &mut Vec<f64>, memory: &mut HashMap<String, f64>, instruction: &String) {
        let value_to_pop: Vec<&str> = instruction.split(':').collect();
        if let Some(value) = value_to_pop.get(1) {
            let value_to_pop = stack.pop().unwrap();
            memory.insert(value.to_string(), value_to_pop);
        };
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        let regex = Regex::new(r"stv:[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        regex.is_match(instruction)
    }
}