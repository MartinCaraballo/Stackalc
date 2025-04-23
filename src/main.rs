mod instruction;

use instruction::{Add, Ceq, Cgt, Clt, Div, Dup, Ldc, Mul, Neg, Pop, Sub, Clear};
use crate::instruction::Instruction;

fn main() {
    let stack: &mut Vec<f64> = &mut Vec::new();

    let add = Add::default();
    let ceq = Ceq::new(add);
    let cgt = Cgt::new(ceq);
    let clt = Clt::new(cgt);
    let div = Div::new(clt);
    let dup = Dup::new(div);
    let ldc = Ldc::new(dup);
    let mul = Mul::new(ldc);
    let neg = Neg::new(mul);
    let pop = Pop::new(neg);
    let sub = Sub::new(pop);
    let mut clear = Clear::new(sub);

    loop {
        let mut instructions = String::new();
        std::io::stdin().read_line(&mut instructions).unwrap();
        let instructions: Vec<&str> = instructions.split(' ').collect();

        for instruction in instructions {
            clear.execute(stack, &instruction.trim().to_string());
        }
        println!("{:?}", stack)
    }
}
