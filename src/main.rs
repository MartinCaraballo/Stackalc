mod instruction;

use std::collections::HashMap;
use instruction::{Add, Ceq, Cgt, Clt, Div, Dup, Ldc, Mul, Neg, Pop, Sub, Clear, Ldv, Stv, Nop, Rng};
use crate::instruction::Instruction;

fn main() {
    let stack: &mut Vec<f64> = &mut Vec::new();
    let memory: &mut HashMap<String, f64> = &mut HashMap::new();

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
    let clear = Clear::new(sub);
    let ldv = Ldv::new(clear);
    let stv = Stv::new(ldv);
    let nop = Nop::new(stv);
    let mut rng = Rng::new(nop);

    loop {
        let mut instructions = String::new();
        std::io::stdin().read_line(&mut instructions).unwrap();
        let instructions: Vec<&str> = instructions.split(' ').collect();

        for instruction in instructions {
            rng.execute(stack, memory, &instruction.trim().to_string());
            if instruction.eq("ret") {
                break;
            }
        }
        println!("{:?}", stack)
    }
}
