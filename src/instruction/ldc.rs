use std::iter::Map;
use super::{into_next, Instruction};
use regex::Regex;

#[derive(Default)]
pub struct Ldc {
    next: Option<Box<dyn Instruction>>,
}

impl Ldc {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Ldc {
    fn handle(&mut self, stack: &mut Vec<f64>, _memory: &mut Map<str, f64>, instruction: &String) {
        let value_to_push: Vec<&str> = instruction.split(':').collect();

        if let Some(value) = value_to_push.get(1) {
            match value.parse::<f64>() {
                Ok(value) => {
                    stack.push(value);
                }
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        let regex = Regex::new(r"^ldc:(\d+(.\d+)?)$").unwrap();
        regex.is_match(instruction)
    }
}
