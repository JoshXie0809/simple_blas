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
            Box::new([0.0, 1.0, 0.0, 3.0]), (2, 2), true
        )?;

        let det: f32 = arr1.mdet()?;
        // true is zero
        let tol = 1e-10;
    
        assert!( det.abs() < tol);
  
        Ok(())
    }

    #[test]
    fn mat_det_array_2d_3() -> Result<(), ListError>{

        // only SQUARE matrix can calc det

        let arr1: Array<f64> = Array::new_array_2d(
            Box::new([31.0, 0.0, 0.0, 
                          2.0, 45.0, 0.0,
                          0.0, 14.2, -9.0]), (3,3), false
        )?;

        let det: f64 = arr1.mdet()?;
        let true_val: f64 = 31.0 * 45.0 * (-9.0);
        let tol: f64 = 1e-15;

        assert!( (det - true_val).abs() < tol );
        println!("{}", (det - true_val).abs());
 
        Ok(())
    }

    #[test]
    fn mat_det_array_2d_4() -> Result<(), ListError>{

        // only SQUARE matrix can calc det

        let arr1: Array<f64> = Array::new_array_2d(
            Box::new([1.0, 4.0, 7.0, 
                          2.0, 5.0, 8.0,
                          3.0, 6.0, 9.0]), (3,3), true
        )?;

        let det: f64 = arr1.mdet()?;
        let true_val: f64 = 0.0;
        let tol: f64 = 1e-10;

        assert!( (det - true_val).abs() < tol );
 
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