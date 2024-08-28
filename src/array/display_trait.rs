use std::{any::type_name, fmt::{self, LowerExp}, ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign}};
use super::Array;

impl<T> fmt::Display for Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default + Sub<Output=T> + PartialOrd + From<f32> + LowerExp
+ fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_scalar<T> (val: T, f: &mut fmt::Formatter<'_>) -> fmt::Result
        where
            T: PartialOrd + From<f32> + LowerExp + 
            fmt::Display + Copy + Default +  Sub<Output=T> ,
        {
            let zero: T = T::default();
            let absval: T = if val < zero { zero - val } else { val };

            if absval >= T::from(1e4_f32) || absval < T::from(1e-4_f32) {
                write!(f, "{:10.3e}, ", val)
            } else {
                write!(f, "{:10.4}, ", val)
            }
        }

        match self {
            Array::Null => {
                write!(f, "Null of type {}", type_name::<T>())
            },

            Array::Scalar(val) => {        
                print_scalar(*val, f)
            },

            Array::Array1D { arr } => {
                let length: usize = arr.len();
                let mut c: usize = 0;
                while c < length {
                    let val = arr[c];
                    if c % 6 == 0 {let _ = write!(f, "\n");}
                    let _ = print_scalar(val, f);
                    c += 1;
                }
                write!(f, "")
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

    #[test]
    fn display_test_arr_1d() {
        let arr: Vec<f64> = vec![
            1.05555555, 23245325342.0, 0.000000013455,
            1.05755, 2322.07, 0.000013455,
            1.0255, 235342.0, 0.0013455,
            1.055, 2325342.0, 0.000013455,
            1.055555, 2325342.0, 0.000000013455,
        ];
        
        let arr: Array<f64> = Array::new_array_1d(arr.into_boxed_slice());
        println!("{}", arr);
    }

}