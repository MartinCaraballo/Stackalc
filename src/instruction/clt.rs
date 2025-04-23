use super::{into_next, Instruction};

#[derive(Default)]
pub struct Clt {
    next: Option<Box<dyn Instruction>>,
}

impl Clt {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Clt {

    fn handle(&mut self, stack: &mut Vec<f64>, _instruction: &String) {
        let first_value = stack.pop().unwrap();
        let second_value = stack.pop().unwrap();

        if first_value < second_value {
            stack.push(1.0)
        } else {
            stack.push(0.0)
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("clt")
    }
}