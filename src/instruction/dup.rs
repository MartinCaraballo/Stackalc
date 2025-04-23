use std::ops::Mul;
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

    fn handle(&mut self, stack: &mut Vec<f64>, instruction: &String) {
        let top = stack.last().unwrap();

        stack.push(top.mul(2))
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("dup")
    }
}