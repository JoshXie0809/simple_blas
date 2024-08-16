use std::ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign};

mod add_method;
mod minus_method;
mod mult_method;
mod div_method;
mod convert_method;

mod index_trait;

pub(crate) fn idxr(r: usize, c: usize, dim: (usize, usize)) -> usize {
    r * dim.1 + c
}

pub(crate) fn idxc(r: usize, c: usize, dim: (usize, usize)) -> usize {
    c * dim.0 + r
}


/// ## Possibe Error types
#[derive(Debug, PartialEq)]
pub enum ListError {
    DivisionByZero,
    MismatchedTypes,
    DifferentLength1D,
    MismatchedDim
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
/// 
/// let arr_1d = Array::new_array_1d(Box::new([1, 2, 5, 7, 0_i32]));
/// println!("{:?}", arr_1d);
/// ```

#[derive(Debug)]
pub enum Array<T>
where T:
Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    Null,
    Scalar(T),
    // Vector
    Array1D { arr: Box<[T]> },

    // Matrix 
    // nr: number of rows
    // nc: number of columns
    Array2D { 
        arr: Box<[T]>, 
        nr: usize, nc: usize, 
        put_val_by_row: bool 
    }
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

    pub fn new_array_2d(arr: Box<[T]>, dim: (isize, isize), put_val_by_row: bool) -> Result<Self, ListError> {
        let mut dim_u = (0_usize, 0_usize);
        match dim {
            (x, _)  if x < 0 => {
                dim_u.0 = (-x) as usize;
                dim_u.1 = arr.len() / dim_u.0;
            },

            (_, y)  if y < 0 => {
                dim_u.1 = (-y) as usize;
                dim_u.0 = arr.len() / dim_u.1;
            },

            _ => {
                dim_u.0 = dim.0 as usize;
                dim_u.1 = dim.1 as usize;
            },
        }
        
        if (dim_u.0 * dim_u.1) != arr.len() {
            return Err(ListError::MismatchedDim);
        }

        Ok(
            Array::Array2D { arr, nr: dim_u.0, nc: dim_u.1, put_val_by_row }
        )
    }

}

impl<T> PartialEq for Array<T>
where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> 
+ PartialEq + AddAssign + Copy + MulAssign + SubAssign
+ Default
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Self::Null) => false,
            (Self::Null, _) => false,

            (Self::Scalar(x), Self::Scalar(y)) => *x == *y,
            
            (Self::Array1D { arr: arr1 }, 
             Self::Array1D { arr: arr2 })
                => arr1 == arr2,
            
            (Self::Array2D { arr: arr1, nr: nr1, nc: nc1, put_val_by_row: by_row1 },
             Self::Array2D { arr: arr2, nr: nr2, nc: nc2, put_val_by_row: by_row2 }
            ) => {
                if (nr1, nc1) != (nr2, nc2) {return  false;}
                if by_row1 == by_row2 {
                   return arr1 == arr2;
                }

                for r in 0..(*nr1) {
                    for c in 0..(*nr2) {
                        if self[(r, c)] != other[(r, c)] {
                            return false;
                        }
                    }
                }

                true
            }, 
            
            _ =>  panic!("cannot compare!!"),
        }
    }
}