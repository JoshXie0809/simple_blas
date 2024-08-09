#[cfg(test)]
pub mod tests {
    
    use simple_blas::Array;
    use simple_blas::ListError;

    #[test]
    fn mult_method_scalar() -> Result<(), ListError> {
        let mut scalar: Array<f64> = Array::new_scalar(3.0178);
        let other_scalar: Array<f64> = Array::new_scalar(3.0178);
        scalar.mult(&other_scalar)?;
        assert_eq!(scalar, Array::new_scalar(3.0178 * 3.0178));
        Ok(())
    }

    #[test]
    fn mult_method_array_1d() -> Result<(), ListError> {
        let a: Box<[i32; 3]> = Box::new([1, 2, 3]);
        let b: Box<[i32; 3]> = Box::new([4, 5, 6]);

        let mut arr_1d: Array<i32> = Array::new_array_1d(a);
        let other_arr_1d: Array<i32> = Array::new_array_1d(b);
        
        arr_1d.mult(&other_arr_1d)?;
        assert_eq!(arr_1d, Array::new_array_1d(Box::new([4, 10, 18])) );
        
        let sclar = Array::new_scalar(2);
        
        arr_1d.mult(&sclar)?;
        assert_eq!(arr_1d, Array::new_array_1d( Box::new([8, 20, 36]) ));

        let other_arr_1d = 
            Array::new_array_1d(Box::new([1, 2, 3, 4, 5, 6]));
        if let Err(err) = arr_1d.mult(&other_arr_1d) {
            assert_eq!(err, ListError::DifferentLength1D)
        };

        Ok(())
    }

}