use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

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

#[derive(Debug, PartialEq)]
pub enum ListError {
    DivisionByZero,
    MismatchedTypes,
    DifferentLength1D,
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

    pub fn add(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x += *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 }) => {
                let len1 = arr1.len();
                if len1 != arr2.len() {
                    return Err(ListError::DifferentLength1D);
                }

                for i in 0..(len1) {
                    arr1[i] += arr2[i];
                }
            },

            (Self::Array1D { arr: arr1 }, 
             Self::Scalar(val)) => {
                let len1 = arr1.len();
                for i in 0..(len1) {
                    arr1[i] += *val;
                }
            },
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    pub fn minus(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x -= *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 }) => {
                let len1 = arr1.len();
                if len1 != arr2.len() {
                    return Err(ListError::DifferentLength1D);
                }

                for i in 0..(len1) {
                    arr1[i] -= arr2[i];
                }
            },

            (Self::Array1D { arr: arr1 }, 
             Self::Scalar(val)) => {
                let len1 = arr1.len();
                for i in 0..(len1) {
                    arr1[i] -= *val;
                }
            },

            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    pub fn mult(&mut self, other: &Self) -> Result<(), ListError>{
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x *= *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 }) => {
                let len1 = arr1.len();
                if len1 != arr2.len() {
                    return Err(ListError::DifferentLength1D);
                }

                for i in 0..(len1) {
                    arr1[i] *= arr2[i];
                }
            },
    
            (Self::Array1D { arr: arr1 }, 
             Self::Scalar(val)) => {
                let len1 = arr1.len();
                for i in 0..(len1) {
                    arr1[i] *= *val;
                }
            },
            
            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    pub fn div(&mut self, other: &Self) -> Result<(), ListError> {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => 
                {
                    if *y == T::default() {
                        return Err(ListError::DivisionByZero);
                    }
                    *x *= *y
                },
            
            _ => return Err(ListError::MismatchedTypes),
        }

        Ok(())
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




