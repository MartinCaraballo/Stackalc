use super::{into_next, Instruction};

#[derive(Default)]
pub struct Mul {
    next: Option<Box<dyn Instruction>>,
}

impl Mul {
    pub fn new(next: impl Instruction + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Instruction for Mul {

    fn handle(&mut self, stack: &mut Vec<f64>, instruction: &String){
        let first_value = stack.pop().unwrap();
        let second_value = stack.pop().unwrap();
        let result = first_value * second_value;

        stack.push(result);
    }
    
    fn next(&mut self) -> &mut Option<Box<dyn Instruction>>{
        &mut self.next
    }

    fn can_handle(&mut self, instruction: &String) -> bool {
        instruction.to_lowercase().eq("mul")
    }

}