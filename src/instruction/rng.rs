use std::collections::HashMap;
use rand::Rng as random;
use super::{into_next, Instruction};

#[derive(Default)]
pub struct Rng {
    next: Option<Box<dyn Instruction>>,
}

impl Rng {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Rng {
    fn handle(&mut self, stack: &mut Vec<f64>, _memory: &mut HashMap<String, f64>, _instruction: &String) {
        let random_number = rand::rng().random_range(0.0..1.0);
        stack.push(random_number);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("rng")
    }
}