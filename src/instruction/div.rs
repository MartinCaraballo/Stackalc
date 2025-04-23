use super::{into_next, Instruction};

#[derive(Default)]
pub struct Div {
    next: Option<Box<dyn Instruction>>,
}

impl Div {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Div {

    fn handle(&mut self, stack: &mut Vec<f64>, _instruction: &String) {
        let first_value = stack.pop().unwrap();
        let second_value = stack.pop().unwrap();
        let result = first_value / second_value;

        stack.push(result);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("div")
    }
}