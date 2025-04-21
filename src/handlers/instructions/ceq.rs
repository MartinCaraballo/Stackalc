use crate::handlers::instruction::Instruction;
use crate::stackalc::Stack;

#[derive(Default)]
pub struct Ceq {
    pub next: Option<Box<dyn Instruction>>,
}

impl Instruction for Ceq {
    fn handle(&mut self, stack: &mut Stack, instruction: String) {
        if instruction.eq("ceq") {
            let first_value = stack.stack.pop().unwrap().to_string().parse::<f64>().unwrap();
            let second_value = stack.stack.pop().unwrap().to_string().parse::<f64>().unwrap();

            if first_value == second_value {
                stack.stack.push(1.0);
            } else {
                stack.stack.push(0.0);
            }
        }
    }

    fn next(mut self) -> &mut Option<Box<dyn Instruction>> {
        &mut self.next
    }
}