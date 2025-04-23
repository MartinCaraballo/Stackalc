use std::iter::Map;
use super::{into_next, Instruction};

#[derive(Default)]
pub struct Dup {
    next: Option<Box<dyn Instruction>>,
}

impl Dup {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Dup {

    fn handle(&mut self, stack: &mut Vec<f64>, _memory: &mut Map<str, f64>, _instruction: &String) {
        let top = stack.last().unwrap();

        stack.push(top * 2.0)
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("dup")
    }
}