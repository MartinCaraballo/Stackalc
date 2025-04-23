use super::{into_next, Instruction};

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
    fn handle(&mut self, stack: &mut Vec<f64>, instruction: &String) {
        let value_to_push: Vec<&str> = instruction.split(':').collect();

        let value_to_push = value_to_push
            .get(1)
            .expect("Bad syntax for ldc instruction");

        let value_to_push = value_to_push.parse::<f64>().unwrap();

        stack.push(value_to_push);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("ldc")
    }
}
