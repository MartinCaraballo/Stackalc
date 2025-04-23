use std::collections::HashMap;
use super::{into_next, Instruction};

#[derive(Default)]
pub struct Nop {
    next: Option<Box<dyn Instruction>>,
}

impl Nop {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Nop {
    fn handle(&mut self, _stack: &mut Vec<f64>, _memory: &mut HashMap<String, f64>, _instruction: &String) {
        // no hace nada
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("nop")
    }
}