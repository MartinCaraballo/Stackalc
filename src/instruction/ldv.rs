use std::collections::HashMap;
use regex::Regex;
use super::{into_next, Instruction};

#[derive(Default)]
pub struct Ldv {
    next: Option<Box<dyn Instruction>>,
}

impl Ldv {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Ldv {
    fn handle(&mut self, stack: &mut Vec<f64>, memory: &mut HashMap<String, f64>, instruction: &String) {
        let value_to_push: Vec<&str> = instruction.split(':').collect();

        if let Some(value) = value_to_push.get(1) {
            if let Some(memory_value) = memory.get(*value) {
                stack.push(memory_value.clone());
                memory.remove(*value);
            };
        };
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        let regex = Regex::new(r"ldv:[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        regex.is_match(instruction)
    }
}