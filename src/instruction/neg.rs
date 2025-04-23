use super::{into_next, Instruction};

#[derive(Default)]
pub struct Neg {
    next: Option<Box<dyn Instruction>>,
}

impl Neg {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Neg {

    fn handle(&mut self, stack: &mut Vec<f64>, instruction: &String) {
        let value = stack.pop().unwrap();

        stack.push(-value)
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("neg")
    }
}