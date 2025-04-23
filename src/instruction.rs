mod add;
mod ceq;
mod cgt;
mod clt;
mod div;
mod dup;
mod ldc;
mod mul;
mod neg;
mod pop;
mod sub;
mod clear;
mod ldv;
mod stv;

use std::collections::HashMap;
pub use add::Add;
pub use ceq::Ceq;
pub use cgt::Cgt;
pub use clt::Clt;
pub use div::Div;
pub use dup::Dup;
pub use ldc::Ldc;
pub use mul::Mul;
pub use neg::Neg;
pub use pop::Pop;
pub use sub::Sub;
pub use clear::Clear;
pub use ldv::Ldv;
pub use stv::Stv;

pub trait Instruction {
    fn execute(&mut self, stack: &mut Vec<f64>, memory: &mut HashMap<String, f64>, instruction: &String) {
        if self.can_handle(instruction) {
            self.handle(stack, memory, instruction);

        } else {
            if let Some(next) = &mut self.next() {
                next.execute(stack, memory, instruction);
            }
        }
    }

    fn handle(&mut self, stack: &mut Vec<f64>, memory: &mut HashMap<String, f64>, instruction: &String);
    fn next(&mut self) -> &mut Option<Box<dyn Instruction>>;
    fn can_handle(&mut self, instruction: &String) -> bool;
}

pub fn into_next(instruction: impl Instruction + Sized + 'static) -> Option<Box<dyn Instruction>> {
    Some(Box::new(instruction))
}