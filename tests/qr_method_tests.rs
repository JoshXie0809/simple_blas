#[cfg(test)]
pub mod test {
    use simple_blas::array::{Array, ListError};

    #[test]
    fn qr_arr_2d_1() {
        let arr = Array::Scalar(1.0);
        if let Err(error) = arr.mqr() {
            assert_eq!(error, ListError::MismatchedTypes);
        };
    }

    #[test]
    fn qr_arr_2d_2() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                194.0, 14.0, 
                245.0, 29.0,
            ]), 
            (2, 2), 
            true
        )?;

        let (mut q, r) = arr.mqr()?;

        println!("q: {:?}", q);
        println!("r: {:?}", r);

        q.mmult(&r)?;
        
        println!("q: {:?}", q);
        let dist = Array::compute_dist(&arr, &q)?;
        assert!(dist < 1e-10);
        
        Ok(())
    }

    #[test]
    fn qr_arr_2d_3() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                1.01, 2.02, 3.03, 
                -4.05, 5.01, 6.97
            ]), 
            (2, 3), 
            false
        )?;

        println!("{:?}", arr);

        let (mut q, r) = arr.mqr()?;

        println!("q: {:?}", q);
        println!("r: {:?}", r);

        q.mmult(&r)?;

        let dist = Array::compute_dist(&arr, &q)?;
        println!("q: {:?}", q);
        assert!(dist < 1e-10);
        
        Ok(())
    }

    #[test]
    fn qr_arr_2d_4() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                1.01, 2.02, 
                3.03, -4.05, 
                5.01, 6.97
            ]), 
            (3, 2), 
            true
        )?;

        let (mut q, r) = arr.mqr()?;
        println!("q: {:?}", q);
        println!("r: {:?}", r);

        q.mmult(&r)?;

        let dist = Array::compute_dist(&arr, &q)?;
        println!("{:?}", q);
        assert!(dist < 1e-10);
        Ok(())
    }

    #[test]
    fn qr_householder_arr_2d_1() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                1.01, 2.02, 
                3.03, -4.05, 
                5.01, 6.97
            ]), 
            (3, 2), 
            true
        )?;

        let (_q, _r) = arr.mqr_householder()?;
        Ok(())
    }

    #[test]
    fn qr_householder_arr_2d_2() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                1.01, 2.02, 
                3.03, -4.05, 
                5.01, 6.97
            ]), 
            (3, 2), 
            true
        )?;

        let (q, mut r) = arr.mqr_householder()?;
        Array::mq_factor_mult_mat_a(&q, &mut r)?;
        let d = Array::compute_dist(&arr, &r)?;
        assert!(d < 1e-10);
        Ok(())
    }

    #[test]
    fn qr_householder_arr_2d_3() -> Result<(), ListError> {
        let arr = Array::new_array_2d(
            Box::new([
                -1.03, 4.023, 
                17.03, -41.05, 
                5.01, 6.97,
                11.01, -9.23,
            ]), 
            (4, 2), 
            true
        )?;

        let (mut q, mut r) = arr.mqr_householder()?;
        q.reverse();
        r.transpose()?;

        Array::mmat_a_mult_q_factor( &mut r, &q)?;

        r.transpose()?;
        
        let d = Array::compute_dist(&arr, &r)?;
        println!("{:e}", d);
        assert!(d < 1e-10);
        Ok(())
    }
}

