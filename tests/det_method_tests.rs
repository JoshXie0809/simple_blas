#[cfg(test)]
pub mod tests {
    use simple_blas::array::{Array, ListError};

    // calc determinat for a matrix 
    // !!!!!!!!!!!!!!!!!!!!!!!!!
    // !!! only for f32, f64 !!!
    // !!!!!!!!!!!!!!!!!!!!!!!!!

    #[test]
    fn mat_det_array_2d_1() -> Result<(), ListError>{

        // only SQUARE matrix can calc det

        let arr1: Array<f64> = Array::new_array_2d(
            Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), (2,3), true
        )?;

        if let Err(error) = arr1.mdet() {
            assert_eq!(error, ListError::MatrixDetDimError)
        };
 
        Ok(())
    }

    #[test]
    fn mat_det_array_2d_2() -> Result<(), ListError>{

        // only SQUARE matrix can calc det

        let arr1: Array<f32> = Array::new_array_2d(
            Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), (2,3), true
        )?;

        if let Err(error) = arr1.mdet() {
            assert_eq!(error, ListError::MatrixDetDimError)
        };
 
        Ok(())
    }

    #[test]
    fn permutation_array_2d() -> Result<(), ListError> {
        let mut arr = Array::new_array_2d(
            Box::new([3, 4, 1, 2, -5, -6]), (3, 2), true)?;
        
        let _p: Vec<(usize, usize)> = arr.permute()?;

        assert_eq!(
            arr,
            Array::new_array_2d(Box::new([-5, -6, 3, 4, 1, 2]), (3, 2), true)?
        );

        Ok(())
    }
}