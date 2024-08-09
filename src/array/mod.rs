use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

mod add_method;
mod minus_method;
mod mult_method;
mod div_method;


/// ## Possibe Error types
#[derive(Debug, PartialEq)]
pub enum ListError {
    DivisionByZero,
    MismatchedTypes,
    DifferentLength1D,
}


/// ## Three possible Type (Now, Matrix will add later)
/// -1 Null: store nothing
/// 
/// -2 Scalar: store a value
/// 
/// -3 Array1D: store 1 dimension vector
/// ```
/// use simple_blas::array::Array;
/// 
/// // create a Null and specify the type
/// // for add value later
/// let null = Array::<i32>::new_null();
/// 
/// // create a scalar that stores a value
/// let scalar = Array::new_scalar(123_i32);
/// 
/// // Scalar(123)
/// println!("{:?}", scalar);
/// ```

#[derive(Debug)]
pub enum Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    Null,
    Scalar(T),
    Array1D{ arr: Box<[T]> },
}

impl<T> Array<T> 
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    pub fn new_null() -> Self {
        Self::Null
    }

    pub fn new_scalar(val: T) -> Self {
        Self::Scalar(val)
    }

    pub fn new_array_1d(arr: Box<[T]>) -> Self {
        Self::Array1D { 
            arr
        }
    }
}

impl<T> PartialEq for Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) => *x == *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 })
                => arr1 == arr2,

            (_, Self::Null) => false,
            (Self::Null, _) => false,

            _ =>  panic!("cannot compare!!"),
        }
        
    }
}
