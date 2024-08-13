#[cfg(test)]
pub mod tests {

    use simple_blas::array::{Array, ListError};
    
    #[test]
    fn arr_2d_index() -> Result<(), ListError> {
        let mut arr = 
            Array::new_array_2d(Box::new([1, 2, 3, 4, 5, 6]), (1, 6), true)?;
        
        arr[(0, 5)] += 10000;

        assert_eq!(arr, 
            Array::new_array_2d(Box::new([1, 2, 3, 4, 5, 10006]), (1, 6), true)?
        );

        Ok(())
    }
    
    #[test]
    fn arr_1d_index() {
        let arr = Array::new_array_1d(Box::new([0, 1, 2, 3]));
        let idx = 1_usize;
        assert_eq!(arr[idx], 1);
    }

    #[test]
    fn arr_1d_index_mut() {
        let mut arr = Array::new_array_1d(Box::new([0, 1, 2, 3]));
        let idx = 1_usize;
        assert_eq!(arr[idx], 1);

        arr[idx] += 54087;
        assert_eq!(arr, 
            Array::new_array_1d(Box::new([0, 54088, 2, 3]))
        );

    }
 
    #[test]
    #[should_panic]
    fn arr_1d_index_bound_check() {
        let arr = Array::new_array_1d(Box::new([0, 1, 2, 3]));
        // unaccess index
        let idx = 12_usize;
        arr[idx];
    }

    #[test]
    fn scalar_index() {
        
        let mut scalar = Array::Scalar(1);
        assert_eq!(scalar[0], 1);
        
        scalar[0] = 123;
        assert_eq!(scalar, Array::Scalar(123));
    }

    #[test]
    #[should_panic]
    fn null_index() {
        // other variant cannot use Index
        let null = Array::<i32>::new_null();
        null[0];
    }
}