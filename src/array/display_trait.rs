use std::{any::type_name, fmt::{self, LowerExp}, ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign}};
use super::Array;


impl<T> fmt::Display for Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + Sub<Output=T> + PartialOrd + From<f32> + LowerExp
+ fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Array::Null => {
                write!(f, "Null of type {}", type_name::<T>())
            },

            Array::Scalar(val) => {        
                let z: T = T::default();
                let absval: T = Array::abs(*val, z);
                if absval >= T::from(1e8_f32) || absval < T::from(1e-4_f32) {
                    write!(f, "{:9.3e}", val)
                } else {
                   write!(f, "{:9.4}", val)
                }
            },

            _ => {write!(f, "??")},
        }
    }
}


#[cfg(test)]
pub mod tests{
    use crate::array::Array;

    #[test]
    fn display_test_null() {
        let null: Array<f32> = Array::new_null();
        println!("{:?}", null);
        println!("{}", null);
    }

    #[test]
    fn display_test_scalar() {
        let scalar1: Array<f64> = Array::new_scalar(124554453.05565645);
        let scalar2: Array<f64> = Array::new_scalar(0.000006556979794);
        println!("{:?}, {:?}", scalar1, scalar2);
        println!("{}, {}", scalar1, scalar2);
    }

}