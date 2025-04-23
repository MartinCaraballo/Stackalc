use super::{into_next, Instruction};

#[derive(Default)]
pub struct Pop {
    next: Option<Box<dyn Instruction>>,
}

impl Pop {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Pop {
    fn handle(&mut self, stack: &mut Vec<f64>, _instruction: &String) {
        stack.pop();
    }
    
    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("pop")
    }
    
}