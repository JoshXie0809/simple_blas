#[cfg(test)]
pub mod tests {
    use simple_blas::array::{Array, ListError};
    // use simple_blas::array::ListError;

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
    fn new_a_array_2d() -> Result<(), ListError>{
        let arr = Box::new([1, 2, 3, 4, 5, 6]);
        let dim = (2, 3);
        let put_val_by_row = true;
        let arr_2d = Array::new_array_2d(arr, dim, put_val_by_row)?;

        let other_arr_2d = Array::Array2D {
             arr: Box::new([1, 2, 3, 4, 5, 6]), 
             nr: 2_usize, 
             nc: 3_usize,
             put_val_by_row
        };

        assert_eq!(arr_2d, other_arr_2d);

        let other_arr_2d = Array::Array2D {
            arr: Box::new([1, 2, 3, 4, 5, 6]), 
            nr: 2_usize, 
            nc: 3_usize,
            put_val_by_row: false
       };

       assert_ne!(arr_2d, other_arr_2d);

       let other_arr_2d = Array::Array2D {
            arr: Box::new([1, 4, 2, 5, 3, 6]), 
            nr: 2_usize, 
            nc: 3_usize,
            put_val_by_row: false
        };

        assert_eq!(arr_2d, other_arr_2d);

        Ok(())
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
}