mod instruction;

use instruction::{Add, Ceq, Cgt, Clt, Div, Dup, Ldc, Mul, Neg, Pop, Sub};

fn main() {
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

    

}
