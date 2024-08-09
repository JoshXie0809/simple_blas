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

            _ => {return Err(ListError::MismatchedTypes)},
        }

        Ok(())
    }

    pub fn mult(&mut self, other: &Self) -> Result<(), ListError>{
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) 
                => *x *= *y,
            
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_a_scalar() {
        let result = Array::new_scalar(123);
        assert_eq!(result, Array::Scalar(123));
    }

    #[test]
    fn new_a_array_1d() {
        let arr1 = Box::new([1, 2, 3, 4, 5]);
        let result = Array::new_array_1d(arr1);
        
        let other_arr = 
            Array::Array1D { arr: Box::new([1, 2, 3, 4, 5]) };
        assert_eq!(result, other_arr);

        let arr1 = Box::new([1.023, 1.1123]);
        let result = Array::new_array_1d(arr1);
        let other_arr = 
            Array::Array1D { arr: Box::new([1.023, 1.1123]) };
        assert_eq!(result, other_arr);
    }

    #[test]
    fn compare_null_and_null() {
        let a: Array<i32> = Array::<i32>::new_null();
        let b: Array<i32> =  Array::<i32>::new_null();
        assert_ne!(a, b);
    }

    #[test]
    fn compare_scalar_and_null() {
        assert_ne!(Array::Null, Array::Scalar(123));
    }

    #[test]
    fn compare_scalar_and_scalar() {
        assert_ne!(Array::Scalar(124), Array::Scalar(123));
    }

    #[test]
    fn add_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        let other_scalar = Array::new_scalar(1.0);
        scalar.add(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(2.0));
        Ok(())
    }

    #[test]
    fn add_method_array_1d() -> Result<(), ListError> {
        let a = Box::new([1, 2, 3]);
        let b = Box::new([4, 5, 6]);

        let mut arr_1d = Array::new_array_1d(a);
        let other_arr_1d = Array::new_array_1d(b);
        
        arr_1d.add(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([5, 7, 9])) );
        
        let sclar = Array::new_scalar(1);
        
        arr_1d.add(&sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([6, 8, 10]) ));

        let other_arr_1d = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.add(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

    #[test]
    fn minus_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(1.0);
        let other_scalar = Array::new_scalar(1.0);
        scalar.minus(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(0.0));
        Ok(())
    }

    #[test]
    fn minus_method_array_1d() -> Result<(), ListError> {

        Ok(())
    }

    #[test]
    fn mult_method_scalar() -> Result<(), ListError> {
        let mut scalar = Array::new_scalar(3.0178);
        let other_scalar = Array::new_scalar(3.0178);
        scalar.mult(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(3.0178 * 3.0178));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn div_method_scalar() {
        let mut scalar = Array::new_scalar(1.0);
        let mut other_scalar = Array::new_scalar(0.0);
        
        let _ = other_scalar.div(&scalar);
        assert_eq!(other_scalar, Array::new_scalar(0.0));

        if let Err(error) = scalar.div(&other_scalar) {
            panic!("{:?}", error);
        };
    }


}
