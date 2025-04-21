use crate::stackalc::Stack;

pub trait Instruction {
    fn execute(&mut self, stack: &mut Stack) {
        self.handle(stack);

        if let Some(next) = &mut self.next() {
            next.execute(stack);
        }
    }

    fn handle(&mut self, stack: &mut Stack, instruction: String);
    fn next(&mut self) -> &mut Option<Box<dyn Instruction>>;
}

pub fn into_next(instruction: impl Instruction + Sized + 'static) -> Option<Box<dyn Instruction>> {
    Some(Box::new(instruction)  )
}