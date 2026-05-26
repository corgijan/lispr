use crate::types::Lispex;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_ops {
    ($op:tt,$fnname:ident,$implType:ty,$forType:ty,$patern:pat,$out:expr)=>{

impl $op<$implType> for $forType{
    type Output = Lispex;
    fn $fnname(self,o:$implType)->Lispex{
        if let $patern = (self,o){
            Lispex::NUMBER($out)
        }else{
            panic!(stringify!($implType encountered an error with $op))
        }
    }
}

    };
}

impl_ops!(
    Add,
    add,
    Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a + b
);
impl_ops!(
    Add,
    add,
    Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a + b
);
impl_ops!(
    Add,
    add,
    &Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a + b
);
impl_ops!(
    Add,
    add,
    &Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a + b
);

impl_ops!(
    Sub,
    sub,
    Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a - b
);
impl_ops!(
    Sub,
    sub,
    &Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a - b
);
impl_ops!(
    Sub,
    sub,
    Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a - b
);
impl_ops!(
    Sub,
    sub,
    &Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a - b
);

impl_ops!(
    Mul,
    mul,
    Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a * b
);
impl_ops!(
    Mul,
    mul,
    &Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a * b
);
impl_ops!(
    Mul,
    mul,
    Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a * b
);
impl_ops!(
    Mul,
    mul,
    &Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a * b
);

impl_ops!(
    Div,
    div,
    Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a / b
);
impl_ops!(
    Div,
    div,
    &Lispex,
    Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a / b
);
impl_ops!(
    Div,
    div,
    Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a / b
);
impl_ops!(
    Div,
    div,
    &Lispex,
    &Lispex,
    (Lispex::NUMBER(a), Lispex::NUMBER(b)),
    a / b
);
