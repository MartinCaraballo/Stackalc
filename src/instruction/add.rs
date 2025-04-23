use std::collections::HashMap;
use super::{Instruction};

#[derive(Default)]
pub struct Add {
    next: Option<Box<dyn Instruction>>,
}

impl Instruction for Add {
    fn handle(&mut self, stack: &mut Vec<f64>, _memory: &mut HashMap<String, f64>, _instruction: &String) {
        let first_value = stack.pop().unwrap();
        let second_value = stack.pop().unwrap();
        let result = first_value + second_value;

        stack.push(result);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("add")
    }
}