use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub struct ExprValue(pub f32);

impl Into<f32> for ExprValue {
    fn into(self) -> f32 {
        self.0
    }
}


macro_rules! impl_bin_op {
    ($op:ident, $fun:ident) => {
        impl $op<ExprValue> for ExprValue {
            type Output = ExprValue;
            fn $fun(self, rhs: ExprValue) -> Self::Output {
                ExprValue(self.0.$fun(rhs.0))
            }
        }
    }
}

impl_bin_op!(Add, add);
impl_bin_op!(Sub, sub);
impl_bin_op!(Mul, mul);
impl_bin_op!(Div, div);