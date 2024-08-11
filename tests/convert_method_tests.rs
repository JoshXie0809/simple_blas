#[cfg(test)]
pub mod tests {
    use simple_blas::array::{Array, ListError};

    #[test]
    fn null_convert_to_scalar_test() -> Result<(), ListError> {
        let mut null: Array<i32> = Array::<i32>::new_null();

        null.convert_to_scalar(123456)?;

        assert_eq!(null, Array::Scalar(123456));

        // other variants cannot convert to scalar

        let mut scalar = Array::new_scalar(123);
        if let Err(error) = scalar.convert_to_scalar(1230) {
            assert_eq!(error, ListError::MismatchedTypes)
        };

        let mut arr_1d = Array::new_array_1d(Box::new([1, 2, 3]));
        if let Err(error) = arr_1d.convert_to_scalar(1230) {
            assert_eq!(error, ListError::MismatchedTypes)
        };

        let mut arr_2d = 
            Array::new_array_2d(Box::new([1, 2, 3]), (1, 3), true)?;

        if let Err(error) = arr_2d.convert_to_scalar(1230) {
            assert_eq!(error, ListError::MismatchedTypes)
        };

        Ok(())
    }

    #[test]
    fn null_convert_to_arr_1d_test() -> Result<(), ListError> {
        let mut null = Array::<i32>::new_null();
        null.convert_to_arr_1d(Box::new([1, 2, 3, 4, 5, 6]))?;
        assert_eq!(null, Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6])));
        
        Ok(())
    }

}