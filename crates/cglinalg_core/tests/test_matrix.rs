extern crate cglinalg_trigonometry;
extern crate cglinalg_core;


#[cfg(test)]
mod matrix2x2_tests {
    use cglinalg_trigonometry::{
        Angle,
        Radians,
    };
    use cglinalg_core::{
        Vector2,
        Matrix2x2,
        Unit,
    };
    use approx::{
        assert_relative_eq,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[2][2], matrix[2][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_identity_matrix_times_identity_matrix_equals_identity_matrix() {
        let identity_matrix: Matrix2x2<f32> = Matrix2x2::identity();
        
        assert_eq!(identity_matrix * identity_matrix, identity_matrix);
    }

    #[test]
    fn test_zero_matrix_times_zero_matrix_equals_zero_matrix() {
        let zero_matrix: Matrix2x2<f32> = Matrix2x2::zero();

        assert_eq!(zero_matrix * zero_matrix, zero_matrix);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        // let expected = Matrix2x2::new(
        //     185091.72_f32, 10939.63_f32, 
        //     26935.295_f32, 1623.9266_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix2x2::identity();
        let b_matrix_times_identity = b_matrix * Matrix2x2::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        // let expected = Matrix2x2::new(
        //     185091.72_f32, 10939.63_f32, 
        //     26935.295_f32, 1623.9266_f32
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix2x2::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix2x2::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix2x2::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix2x2::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        // let expected = Matrix2x2::new(
        //     185091.72_f32, 10939.63_f32, 
        //     26935.295_f32, 1623.9266_f32
        // );
        let zero_times_a_matrix = Matrix2x2::zero() * a_matrix;
        let zero_times_b_matrix = Matrix2x2::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix2x2::zero());
        assert_eq!(zero_times_b_matrix, Matrix2x2::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        // let expected = Matrix2x2::new(
        //     185091.72_f32, 10939.63_f32, 
        //     26935.295_f32, 1623.9266_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix2x2::identity();
        let identity_times_a_matrix = Matrix2x2::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix2x2::identity();
        let identity_times_b_matrix = Matrix2x2::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        // let expected = Matrix2x2::new(
        //     185091.72_f32, 10939.63_f32, 
        //     26935.295_f32, 1623.9266_f32
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let a_matrix = Matrix2x2::new(
            80_f32,      23.43_f32,     
            426.1_f32,   23.5724_f32
        );
        let b_matrix = Matrix2x2::new(
            36.84_f32,   427.46894_f32, 
            7.04217_f32, 61.891390_f32
        );
        let expected = Matrix2x2::new(
            185091.72_f32, 10939.63_f32, 
            26935.295_f32, 1623.9266_f32
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        // let expected = Matrix2x2::new(
        //     3943.4304_f32, 0_f32, 
        //     0_f32,         356.89127_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix2x2::identity();
        let b_matrix_times_identity = b_matrix * Matrix2x2::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        // let expected = Matrix2x2::new(
        //     3943.4304_f32, 0_f32, 
        //     0_f32,         356.89127_f32
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix2x2::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix2x2::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix2x2::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix2x2::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        // let expected = Matrix2x2::new(
        //     3943.4304_f32, 0_f32, 
        //     0_f32,         356.89127_f32
        // );
        let zero_times_a_matrix = Matrix2x2::zero() * a_matrix;
        let zero_times_b_matrix = Matrix2x2::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix2x2::zero());
        assert_eq!(zero_times_b_matrix, Matrix2x2::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        // let expected = Matrix2x2::new(
        //     3943.4304_f32, 0_f32, 
        //     0_f32,         356.89127_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix2x2::identity();
        let identity_times_a_matrix = Matrix2x2::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix2x2::identity();
        let identity_times_b_matrix = Matrix2x2::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        // let expected = Matrix2x2::new(
        //     3943.4304_f32, 0_f32, 
        //     0_f32,         356.89127_f32
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let a_matrix = Matrix2x2::new(
            68.32_f32, 0_f32, 
            0_f32,     37.397_f32
        );
        let b_matrix = Matrix2x2::new(
            57.72_f32, 0_f32, 
            0_f32,     9.5433127_f32
        );
        let expected = Matrix2x2::new(
            3943.4304_f32, 0_f32, 
            0_f32,         356.89127_f32
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix2x2::<f32>::identity();
        let identity_transpose = identity.transpose();
            
        assert_eq!(identity, identity_transpose);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector2::new(1_i32, 2_i32);
        let c1 = Vector2::new(3_i32, 4_i32);
        let columns = [c0, c1];
        let expected = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );
        let result = Matrix2x2::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector2::new(1_i32, 2_i32);
        let r1 = Vector2::new(3_i32, 4_i32);
        let rows = [r0, r1];
        let expected = Matrix2x2::new(
            1_i32, 3_i32,
            2_i32, 4_i32
        );
        let result = Matrix2x2::from_rows(&rows);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169_f64;
        let identity = Matrix2x2::identity();
        let expected = Matrix2x2::new(
            c,     0_f64, 
            0_f64, c
        );

        assert_eq!(identity * c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169_f64;
        let identity = Matrix2x2::identity();
        let expected = Matrix2x2::new(
            1_f64 / c, 0_f64, 
            0_f64,     1_f64 / c
        );

        assert_eq!(identity / c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix = Matrix2x2::zero();
        let matrix = Matrix2x2::new(
            36.84_f64, 427.46_f64, 
            7.47_f64,  61.89_f64
        );

        assert_eq!(matrix + zero_matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix = Matrix2x2::zero();
        let matrix = Matrix2x2::new(
            36.84_f64, 427.46_f64, 
            7.47_f64,  61.89_f64
        );

        assert_eq!(zero_matrix + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant() {
        let matrix = Matrix2x2::new(
            1_f64, 2_f64, 
            4_f64, 8_f64
        );
        
        assert_eq!(matrix.determinant(), 0_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix = Matrix2x2::new(
            2_f64,  0_f64,
            5_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 2_f64 * 3_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix = Matrix2x2::new(
            2_f64,  5_f64,
            0_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 2_f64 * 3_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix2x2::new(
            5_f64, 1_f64, 
            1_f64, 5_f64
        );
        let expected = (1_f64 / 24_f64) * Matrix2x2::new(
             5_f64, -1_f64,
            -1_f64,  5_f64
        );
        let result = matrix.inverse().unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_identity_is_invertible() {
        let identity: Matrix2x2<f64> = Matrix2x2::identity();

        assert!(identity.is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix2x2<f64> = Matrix2x2::identity().inverse().unwrap();
        let expected: Matrix2x2<f64> = Matrix2x2::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix = 4_f64 * Matrix2x2::identity();
        let expected = (1_f64 / 4_f64) * Matrix2x2::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix2x2::new(
            1_f32, 2_f32, 
            3_f32, 4_f32
        );
        
        assert!(matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        let matrix = Matrix2x2::new(
            1_f32, 2_f32, 
            4_f32, 8_f32
        );
        
        assert!(!matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix = Matrix2x2::new(
            1_f32, 2_f32, 
            4_f32, 8_f32
        );
        
        assert!(matrix.inverse().is_none());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix = Matrix2x2::new(
            36.84_f64, 427.46_f64, 
            7.47_f64,  61.89_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix2x2::identity();

        assert_relative_eq!(matrix * matrix_inverse, identity, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix = Matrix2x2::new(
            36.84_f64, 427.46_f64, 
            7.47_f64,  61.89_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix2x2::identity();

        assert_relative_eq!(matrix_inverse * matrix, identity, epsilon = 1e-8);        
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix = Matrix2x2::new(
            80_f64,    426.1_f64,
            23.43_f64, 23.5724_f64
        );
        let constant: f64 = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix = Matrix2x2::new(
            80_f64,    426.1_f64, 
            23.43_f64, 23.5724_f64
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();

        assert_eq!(matrix_transpose_inverse, matrix_inverse_transpose);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix = Matrix2x2::new(
            80_f64,    426.1_f64, 
            23.43_f64, 23.5724_f64
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );
        result.swap_columns(0, 1);
        let expected = Matrix2x2::new(
            3_i32, 4_i32, 
            1_i32, 2_i32
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );
        result.swap_rows(0, 1);
        let expected = Matrix2x2::new(
            2_i32, 1_i32, 
            4_i32, 3_i32
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );
        result.swap((0, 0), (1, 1));
        let expected = Matrix2x2::new(
            4_i32, 2_i32, 
            3_i32, 1_i32
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_scale() {
        let matrix = Matrix2x2::from_scale(3_i32);
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_x * 3_i32 + unit_y * 3_i32;
        let result = matrix * Vector2::new(1_i32, 1_i32);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_nonuniform_scale() {
        let matrix = Matrix2x2::from_nonuniform_scale(&Vector2::new(3_i32, 7_i32));
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_x * 3_i32 + unit_y * 7_i32;
        let result = matrix * Vector2::new(1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_x() {
        let shear_factor = 5_i32;
        let matrix = Matrix2x2::from_shear_x(shear_factor);
        let vectors = [
            Vector2::new( 1_i32,  1_i32),
            Vector2::new(-1_i32,  1_i32),
            Vector2::new(-1_i32, -1_i32),
            Vector2::new( 1_i32, -1_i32),
        ];
        let expected = [
            Vector2::new( 1_i32 + shear_factor,  1_i32),
            Vector2::new(-1_i32 + shear_factor,  1_i32),
            Vector2::new(-1_i32 - shear_factor, -1_i32),
            Vector2::new( 1_i32 - shear_factor, -1_i32),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_y() {
        let shear_factor = 5_i32;
        let matrix = Matrix2x2::from_shear_y(shear_factor);
        let vectors = [
            Vector2::new( 1_i32,  1_i32),
            Vector2::new(-1_i32,  1_i32),
            Vector2::new(-1_i32, -1_i32),
            Vector2::new( 1_i32, -1_i32),
        ];
        let expected = [
            Vector2::new( 1_i32,  1_i32 + shear_factor),
            Vector2::new(-1_i32,  1_i32 - shear_factor),
            Vector2::new(-1_i32, -1_i32 - shear_factor),
            Vector2::new( 1_i32, -1_i32 + shear_factor),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    /// Shearing about the line `y == (1 / 2) * x`.
    #[test]
    fn test_from_shear1() {
        let shear_factor = 5_f64;
        let shear_direction = Unit::from_value(Vector2::new(2_f64, 1_f64));
        let normal = Unit::from_value(Vector2::new(-1_f64, 2_f64));
        let matrix = Matrix2x2::from_shear(shear_factor, &shear_direction, &normal);

        let vector_0 = Vector2::new( 1_f64 / f64::sqrt(5_f64),  3_f64 / f64::sqrt(5_f64));
        let vector_1 = Vector2::new(-3_f64 / f64::sqrt(5_f64),  1_f64 / f64::sqrt(5_f64));
        let vector_2 = Vector2::new(-1_f64 / f64::sqrt(5_f64), -3_f64 / f64::sqrt(5_f64));
        let vector_3 = Vector2::new( 3_f64 / f64::sqrt(5_f64), -1_f64 / f64::sqrt(5_f64));

        let rotation_angle = Radians(f64::atan2(1_f64, 2_f64));
        
        assert_relative_eq!(rotation_angle.cos(), 2_f64 / f64::sqrt(5_f64), epsilon = 1e-10);
        assert_relative_eq!(rotation_angle.sin(), 1_f64 / f64::sqrt(5_f64), epsilon = 1e-10);
        
        let rotation = Matrix2x2::from_angle(rotation_angle);
        let vector_parallel_to_x_axis_0 = Vector2::new( 1_f64,  1_f64);
        let vector_parallel_to_x_axis_1 = Vector2::new(-1_f64,  1_f64);
        let vector_parallel_to_x_axis_2 = Vector2::new(-1_f64, -1_f64);
        let vector_parallel_to_x_axis_3 = Vector2::new( 1_f64, -1_f64);
        let rotated_vector_0 = rotation * vector_parallel_to_x_axis_0;
        let rotated_vector_1 = rotation * vector_parallel_to_x_axis_1;
        let rotated_vector_2 = rotation * vector_parallel_to_x_axis_2;
        let rotated_vector_3 = rotation * vector_parallel_to_x_axis_3;

        assert_relative_eq!(rotated_vector_0, vector_0, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_1, vector_1, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_2, vector_2, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_3, vector_3, epsilon = 1e-10);

        let expected_0 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (1_f64 + shear_factor) - 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (1_f64 + shear_factor) + 2_f64 / f64::sqrt(5_f64),
        );
        let expected_1 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (-1_f64 + shear_factor) - 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (-1_f64 + shear_factor) + 2_f64 / f64::sqrt(5_f64),
        );
        let expected_2 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (-1_f64 - shear_factor) + 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (-1_f64 - shear_factor) - 2_f64 / f64::sqrt(5_f64),
        );
        let expected_3 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (1_f64 - shear_factor) + 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (1_f64 - shear_factor) - 2_f64 / f64::sqrt(5_f64),
        );
        let result_0 = matrix * vector_0;
        let result_1 = matrix * vector_1;
        let result_2 = matrix * vector_2;
        let result_3 = matrix * vector_3;

        assert_relative_eq!(result_0, expected_0, epsilon = 1e-10);
        assert_relative_eq!(result_1, expected_1, epsilon = 1e-10);
        assert_relative_eq!(result_2, expected_2, epsilon = 1e-10);
        assert_relative_eq!(result_3, expected_3, epsilon = 1e-10);
    }

    /// Shearing about the line `y == (1 / 2) * x`.
    #[test]
    fn test_from_shear2() {
        let shear_factor = 5_f64;
        let shear_direction = Unit::from_value(Vector2::new(2_f64, 1_f64));
        let normal = Unit::from_value(-Vector2::new(-1_f64, 2_f64));
        let matrix = Matrix2x2::from_shear(shear_factor, &shear_direction, &normal);
    
        let vector_0 = Vector2::new( 1_f64 / f64::sqrt(5_f64),  3_f64 / f64::sqrt(5_f64));
        let vector_1 = Vector2::new(-3_f64 / f64::sqrt(5_f64),  1_f64 / f64::sqrt(5_f64));
        let vector_2 = Vector2::new(-1_f64 / f64::sqrt(5_f64), -3_f64 / f64::sqrt(5_f64));
        let vector_3 = Vector2::new( 3_f64 / f64::sqrt(5_f64), -1_f64 / f64::sqrt(5_f64));
    
        let rotation_angle = Radians(f64::atan2(1_f64, 2_f64));
            
        assert_relative_eq!(rotation_angle.cos(), 2_f64 / f64::sqrt(5_f64), epsilon = 1e-10);
        assert_relative_eq!(rotation_angle.sin(), 1_f64 / f64::sqrt(5_f64), epsilon = 1e-10);
            
        let rotation = Matrix2x2::from_angle(rotation_angle);
        let vector_parallel_to_x_axis_0 = Vector2::new( 1_f64,  1_f64);
        let vector_parallel_to_x_axis_1 = Vector2::new(-1_f64,  1_f64);
        let vector_parallel_to_x_axis_2 = Vector2::new(-1_f64, -1_f64);
        let vector_parallel_to_x_axis_3 = Vector2::new( 1_f64, -1_f64);
        let rotated_vector_0 = rotation * vector_parallel_to_x_axis_0;
        let rotated_vector_1 = rotation * vector_parallel_to_x_axis_1;
        let rotated_vector_2 = rotation * vector_parallel_to_x_axis_2;
        let rotated_vector_3 = rotation * vector_parallel_to_x_axis_3;
    
        assert_relative_eq!(rotated_vector_0, vector_0, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_1, vector_1, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_2, vector_2, epsilon = 1e-10);
        assert_relative_eq!(rotated_vector_3, vector_3, epsilon = 1e-10);
    
        let expected_0 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (1_f64 - shear_factor) - 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (1_f64 - shear_factor) + 2_f64 / f64::sqrt(5_f64),
        );
        let expected_1 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (-1_f64 - shear_factor) - 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (-1_f64 - shear_factor) + 2_f64 / f64::sqrt(5_f64),
        );
        let expected_2 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (-1_f64 + shear_factor) + 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (-1_f64 + shear_factor) - 2_f64 / f64::sqrt(5_f64),
        );
        let expected_3 = Vector2::new(
            (2_f64 / f64::sqrt(5_f64)) * (1_f64 + shear_factor) + 1_f64 / f64::sqrt(5_f64),
            (1_f64 / f64::sqrt(5_f64)) * (1_f64 + shear_factor) - 2_f64 / f64::sqrt(5_f64),
        );
        let result_0 = matrix * vector_0;
        let result_1 = matrix * vector_1;
        let result_2 = matrix * vector_2;
        let result_3 = matrix * vector_3;
    
        assert_relative_eq!(result_0, expected_0, epsilon = 1e-10);
        assert_relative_eq!(result_1, expected_1, epsilon = 1e-10);
        assert_relative_eq!(result_2, expected_2, epsilon = 1e-10);
        assert_relative_eq!(result_3, expected_3, epsilon = 1e-10);
    }

    #[test]
    fn test_from_shear_from_shear_x() {
        let shear_factor = 7_f64;
        let direction = Unit::from_value(Vector2::unit_x());
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix2x2::from_shear_x(shear_factor);
        let result = Matrix2x2::from_shear(shear_factor, &direction, &normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_from_shear_y() {
        let shear_factor = 7_f64;
        let direction = Unit::from_value(Vector2::unit_y());
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix2x2::from_shear_y(shear_factor);
        let result = Matrix2x2::from_shear(shear_factor, &direction, &normal);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_x_axis1() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix2x2::new(
            1_f64,  0_f64, 
            0_f64, -1_f64
        );
        let result = Matrix2x2::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_x_axis2() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let normal = Unit::from_value(-Vector2::unit_y());
        let expected = Matrix2x2::new(
            1_f64,  0_f64, 
            0_f64, -1_f64
        );
        let result = Matrix2x2::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_y_axis1() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix2x2::new(
            -1_f64, 0_f64, 
             0_f64, 1_f64
        );
        let result = Matrix2x2::from_reflection(&normal);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_y_axis2() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let normal = Unit::from_value(-Vector2::unit_x());
        let expected = Matrix2x2::new(
            -1_f64, 0_f64, 
             0_f64, 1_f64
        );
        let result = Matrix2x2::from_reflection(&normal);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the orientation 
    /// of the line segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_from_plane1() {
        let normal = Unit::from_value(
            Vector2::new(f64::sqrt(2_f64)/ 2_f64, -f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix2x2::new(
            0_f64, 1_f64, 
            1_f64, 0_f64
        );
        let result = Matrix2x2::from_reflection(&normal);
        
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the orientation 
    /// of the line segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_from_plane2() {
        let normal = Unit::from_value(
            Vector2::new(-f64::sqrt(2_f64)/ 2_f64, f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix2x2::new(
            0_f64, 1_f64, 
            1_f64, 0_f64
        );
        let result = Matrix2x2::from_reflection(&normal);
            
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_angle() {
        let matrix: Matrix2x2<f64> = Matrix2x2::from_angle(Radians::full_turn_div_4());
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_y;
        let result = matrix * unit_x;

        assert_relative_eq!(result, expected, epsilon = 1e-8);

        let expected = -unit_x;
        let result = matrix * unit_y;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_rotation_between() {
        let unit_x: Vector2<f64> = Vector2::unit_x();
        let unit_y: Vector2<f64> = Vector2::unit_y();
        let expected = Matrix2x2::new(
             0_f64, 1_f64,
            -1_f64, 0_f64,
        );
        let result = Matrix2x2::rotation_between(&unit_x, &unit_y);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_rotation_between_axis() {
        let unit_x: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_x());
        let unit_y: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_y());
        let expected = Matrix2x2::new(
             0_f64, 1_f64,
            -1_f64, 0_f64,
        );
        let result = Matrix2x2::rotation_between_axis(&unit_x, &unit_y);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }
}


#[cfg(test)]
mod matrix3x3_tests {
    use cglinalg_trigonometry::{
        Angle,
        Radians,
    };
    use cglinalg_core::{
        Vector2,
        Vector3,
        Normed,
        Matrix3x3,
        Unit,
        Point2,
        Point3,
    };
    use approx::{
        assert_relative_eq,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
        assert_eq!(matrix[2][0], 7_i32);
        assert_eq!(matrix[2][1], 8_i32);
        assert_eq!(matrix[2][2], 9_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[3][3], matrix[3][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_identity_matrix_times_identity_matrix_equals_identity_matrix() {
        let identity_matrix: Matrix3x3<f32> = Matrix3x3::identity();
        
        assert_eq!(identity_matrix * identity_matrix, identity_matrix);
    }

    #[test]
    fn test_zero_matrix_times_zero_matrix_equals_zero_matrix() {
        let zero_matrix: Matrix3x3<f32> = Matrix3x3::zero();

        assert_eq!(zero_matrix * zero_matrix, zero_matrix);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        // let expected = Matrix3x3::new(
        //     3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
        //     43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
        //     16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix3x3::identity();
        let b_matrix_times_identity = b_matrix * Matrix3x3::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        // let expected = Matrix3x3::new(
        //     3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
        //     43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
        //     16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix3x3::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix3x3::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix3x3::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix3x3::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        // let expected = Matrix3x3::new(
        //     3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
        //     43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
        //     16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        // );
        let zero_times_a_matrix = Matrix3x3::zero() * a_matrix;
        let zero_times_b_matrix = Matrix3x3::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix3x3::zero());
        assert_eq!(zero_times_b_matrix, Matrix3x3::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        // let expected = Matrix3x3::new(
        //     3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
        //     43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
        //     16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix3x3::identity();
        let identity_times_a_matrix = Matrix3x3::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix3x3::identity();
        let identity_times_b_matrix = Matrix3x3::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        // let expected = Matrix3x3::new(
        //     3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
        //     43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
        //     16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let a_matrix = Matrix3x3::new(
            80_f32,     426.1_f32,   43.393_f32, 
            23.43_f32,  23.5724_f32, 1.27_f32, 
            81.439_f32, 12.19_f32,   43.36_f32
        );
        let b_matrix = Matrix3x3::new(
            36.84_f32,     7.04217_f32,  5.74_f32, 
            427.46894_f32, 61.89139_f32, 96.27_f32, 
            152.66_f32,    86.333_f32,   26.71_f32
        );
        let expected = Matrix3x3::new(
            3579.6579_f32,  15933.496_f32,   1856.4281_f32, 
            43487.7660_f32, 184776.9752_f32, 22802.0289_f32, 
            16410.8178_f32, 67409.1000_f32,  7892.1646_f32
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        // let expected = Matrix3x3::new(
        //     3943.4304_f32, 0_f32,         0_f32, 
        //     0_f32,         356.89127_f32, 0_f32, 
        //     0_f32,         0_f32,         528.96067_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix3x3::identity();
        let b_matrix_times_identity = b_matrix * Matrix3x3::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        // let expected = Matrix3x3::new(
        //     3943.4304_f32, 0_f32,         0_f32, 
        //     0_f32,         356.89127_f32, 0_f32, 
        //     0_f32,         0_f32,         528.96067_f32
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix3x3::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix3x3::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix3x3::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix3x3::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        // let expected = Matrix3x3::new(
        //     3943.4304_f32, 0_f32,         0_f32, 
        //     0_f32,         356.89127_f32, 0_f32, 
        //     0_f32,         0_f32,         528.96067_f32
        // );
        let zero_times_a_matrix = Matrix3x3::zero() * a_matrix;
        let zero_times_b_matrix = Matrix3x3::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix3x3::zero());
        assert_eq!(zero_times_b_matrix, Matrix3x3::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        // let expected = Matrix3x3::new(
        //     3943.4304_f32, 0_f32,         0_f32, 
        //     0_f32,         356.89127_f32, 0_f32, 
        //     0_f32,         0_f32,         528.96067_f32
        // );
        let a_matrix_times_identity = a_matrix * Matrix3x3::identity();
        let identity_times_a_matrix = Matrix3x3::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix3x3::identity();
        let identity_times_b_matrix = Matrix3x3::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        // let expected = Matrix3x3::new(
        //     3943.4304_f32, 0_f32,         0_f32, 
        //     0_f32,         356.89127_f32, 0_f32, 
        //     0_f32,         0_f32,         528.96067_f32
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let a_matrix = Matrix3x3::new(
            68.32_f32, 0_f32,      0_f32, 
            0_f32,     37.397_f32, 0_f32, 
            0_f32,     0_f32,      43.393_f32
        );
        let b_matrix = Matrix3x3::new(
            57.72_f32, 0_f32,         0_f32, 
            0_f32,     9.5433127_f32, 0_f32, 
            0_f32,     0_f32,         12.19_f32
        );
        let expected = Matrix3x3::new(
            3943.4304_f32, 0_f32,         0_f32, 
            0_f32,         356.89127_f32, 0_f32, 
            0_f32,         0_f32,         528.96067_f32
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix3x3::<f32>::identity();
        let identity_transpose = identity.transpose();
            
        assert_eq!(identity, identity_transpose);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector3::new(1_i32, 2_i32, 3_i32);
        let c1 = Vector3::new(4_i32, 5_i32, 6_i32);
        let c2 = Vector3::new(7_i32, 8_i32, 9_i32);
        let columns = [c0, c1, c2];
        let expected = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let result = Matrix3x3::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector3::new(1_i32, 2_i32, 3_i32);
        let r1 = Vector3::new(4_i32, 5_i32, 6_i32);
        let r2 = Vector3::new(7_i32, 8_i32, 9_i32);
        let rows = [r0, r1, r2];
        let expected = Matrix3x3::new(
            1_i32, 4_i32, 7_i32,
            2_i32, 5_i32, 8_i32,
            3_i32, 6_i32, 9_i32
        );
        let result = Matrix3x3::from_rows(&rows);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169_f64;
        let identity = Matrix3x3::identity();
        let expected = Matrix3x3::new(
            c,     0_f64, 0_f64, 
            0_f64, c,     0_f64, 
            0_f64, 0_f64, c
        );

        assert_eq!(identity * c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169;
        let identity = Matrix3x3::identity();
        let expected = Matrix3x3::new(
            1_f64 / c, 0_f64,     0_f64, 
            0_f64,     1_f64 / c, 0_f64, 
            0_f64,     0_f64,     1_f64 / c
        );

        assert_eq!(identity / c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix = Matrix3x3::zero();
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );

        assert_eq!(matrix + zero_matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix = Matrix3x3::zero();
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );

        assert_eq!(zero_matrix + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant() {
        let matrix = Matrix3x3::new(
            1_f32, 2_f32, 3_f32, 
            4_f32, 5_f32, 6_f32, 
            4_f32, 5_f32, 6_f32
        );
        
        assert_eq!(matrix.determinant(), 0_f32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix = Matrix3x3::new(
            1_f64,  0_f64,  0_f64,
            5_f64,  2_f64,  0_f64,
            5_f64,  5_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix = Matrix3x3::new(
            1_f64,  5_f64,  5_f64,
            0_f64,  2_f64,  5_f64,
            0_f64,  0_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix3x3::new(
            5_f64, 1_f64, 1_f64,
            1_f64, 5_f64, 1_f64,
            1_f64, 1_f64, 5_f64
        );
        let expected = (1_f64 / 28_f64) * Matrix3x3::new(
             6_f64, -1_f64, -1_f64, 
            -1_f64,  6_f64, -1_f64, 
            -1_f64, -1_f64,  6_f64,
        );
        let result = matrix.inverse().unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_identity_is_invertible() {
        assert!(Matrix3x3::<f64>::identity().is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix3x3<f64> = Matrix3x3::identity().inverse().unwrap();
        let expected: Matrix3x3<f64> = Matrix3x3::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix = 4_f64 * Matrix3x3::identity();
        let expected = (1_f64 / 4_f64) * Matrix3x3::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix3x3::new(
            1_f32, 2_f32, 3_f32, 
            0_f32, 4_f32, 5_f32, 
            0_f32, 0_f32, 6_f32
        );
        
        assert!(matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        let matrix = Matrix3x3::new(
            1_f32, 2_f32, 3_f32, 
            4_f32, 5_f32, 6_f32, 
            4_f32, 5_f32, 6_f32
        );
        
        assert!(!matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix = Matrix3x3::new(
            1_f32, 2_f32, 3_f32, 
            4_f32, 5_f32, 6_f32, 
            4_f32, 5_f32, 6_f32
        );
        
        assert!(matrix.inverse().is_none());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix3x3::identity();

        assert_relative_eq!(matrix * matrix_inverse, identity, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );
        let constant = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();

        assert_eq!(matrix_transpose_inverse, matrix_inverse_transpose);
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix3x3::identity();

        assert_relative_eq!(matrix_inverse * matrix, identity, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix = Matrix3x3::new(
            80_f64,     426.1_f64,   43.393_f64, 
            23.43_f64,  23.5724_f64, 1.27_f64, 
            81.439_f64, 12.19_f64,   43.36_f64
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        result.swap_columns(0, 1);
        let expected = Matrix3x3::new(
            4_i32, 5_i32, 6_i32, 
            1_i32, 2_i32, 3_i32, 
            7_i32, 8_i32, 9_i32
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        result.swap_rows(0, 1);
        let expected = Matrix3x3::new(
            2_i32, 1_i32, 3_i32, 
            5_i32, 4_i32, 6_i32, 
            8_i32, 7_i32, 9_i32
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        result.swap((0, 0), (2, 1));
        let expected = Matrix3x3::new(
            8_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 1_i32, 9_i32
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_scale() {
        let matrix = Matrix3x3::from_scale(3_i32);
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let expected = unit_x * 3 + unit_y * 3_i32 + unit_z * 3_i32;
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_nonuniform_scale() {
        let matrix = Matrix3x3::from_nonuniform_scale(&Vector3::new(3_i32, 5_i32, 7_i32));
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let expected = unit_x * 3_i32 + unit_y * 5_i32 + unit_z * 7_i32;
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_scale_does_not_change_last_coordinate() {
        let matrix = Matrix3x3::from_affine_scale(5_i32);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_nonuniform_scale() {
        let matrix = Matrix3x3::from_affine_nonuniform_scale(&Vector2::new(7_i32, 11_i32));
        let expected = Vector3::new(7_i32, 11_i32, 1_i32);
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_nonuniform_scale_does_not_change_last_coordinate() {
        let matrix = Matrix3x3::from_affine_nonuniform_scale(&Vector2::new(7_i32, 11_i32));
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_x() {
        let shear_x_with_y = 5_i32;
        let shear_x_with_z = 3_i32;
        let matrix = Matrix3x3::from_shear_x(shear_x_with_y, shear_x_with_z);
        let expected = Vector3::new(1_i32 + shear_x_with_y + shear_x_with_z, 1_i32, 1_i32);
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_y() {
        let shear_y_with_x = 5_i32;
        let shear_y_with_z = 3_i32;
        let matrix = Matrix3x3::from_shear_y(shear_y_with_x, shear_y_with_z);
        let expected = Vector3::new(1_i32, 1_i32 + shear_y_with_x + shear_y_with_z, 1_i32);
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_z() {
        let shear_z_with_x = 5_i32;
        let shear_z_with_y = 3_i32;
        let matrix = Matrix3x3::from_shear_z(shear_z_with_x, shear_z_with_y);
        let expected = Vector3::new(1_i32, 1_i32, 1_i32 + shear_z_with_x + shear_z_with_y);
        let result = matrix * Vector3::new(1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x1() {
        let shear_factor = 5_i32;
        let matrix = Matrix3x3::from_affine_shear_x(shear_factor);
        let vectors = [
            Vector3::new( 1_i32,  1_i32, 1_i32),
            Vector3::new(-1_i32,  1_i32, 1_i32),
            Vector3::new(-1_i32, -1_i32, 1_i32),
            Vector3::new( 1_i32, -1_i32, 1_i32),
        ];
        let expected = [
            Vector3::new( 1_i32 + shear_factor,  1_i32, 1_i32),
            Vector3::new(-1_i32 + shear_factor,  1_i32, 1_i32),
            Vector3::new(-1_i32 - shear_factor, -1_i32, 1_i32),
            Vector3::new( 1_i32 - shear_factor, -1_i32, 1_i32),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_affine_shear_x2() {
        let shear_factor = 5_i32;
        let expected = Matrix3x3::new(
            1_i32,        0_i32, 0_i32,
            shear_factor, 1_i32, 0_i32,
            0_i32,        0_i32, 1_i32
        );
        let result = Matrix3x3::from_affine_shear_x(shear_factor);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x_does_not_change_last_coordinate() {
        let shear_factor = 5_i32;
        let matrix = Matrix3x3::from_affine_shear_x(shear_factor);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y1() {
        let shear_factor = 3_i32;
        let matrix = Matrix3x3::from_affine_shear_y(shear_factor);
        let vectors = [
            Vector3::new( 1_i32,  1_i32, 1_i32),
            Vector3::new(-1_i32,  1_i32, 1_i32),
            Vector3::new(-1_i32, -1_i32, 1_i32),
            Vector3::new( 1_i32, -1_i32, 1_i32),
        ];
        let expected = [
            Vector3::new( 1_i32,  1_i32 + shear_factor, 1_i32),
            Vector3::new(-1_i32,  1_i32 - shear_factor, 1_i32),
            Vector3::new(-1_i32, -1_i32 - shear_factor, 1_i32),
            Vector3::new( 1_i32, -1_i32 + shear_factor, 1_i32),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_affine_shear_y2() {
        let shear_factor = 5_i32;
        let expected = Matrix3x3::new(
            1_i32, shear_factor, 0_i32,
            0_i32, 1_i32,        0_i32,
            0_i32, 0_i32,        1_i32
        );
        let result = Matrix3x3::from_affine_shear_y(shear_factor);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y_does_not_change_last_coordinate() {
        let shear_factor = 3_i32;
        let matrix = Matrix3x3::from_affine_shear_y(shear_factor);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_from_affine_shear_x() {
        let shear_factor = 5_f64;
        let origin = Point2::new(0_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_x());
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix3x3::from_affine_shear_x(shear_factor);
        let result = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_from_affine_shear_y() {
        let shear_factor = 5_f64;
        let origin = Point2::new(0_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_y());
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix3x3::from_affine_shear_y(shear_factor);
        let result = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear1() {
        let shear_factor = 7_f64;
        let origin = Point2::new(-2_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_x());
        let normal = Unit::from_value(Vector2::unit_y());
        let matrix = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);
        let vectors = [
            Vector3::new( 1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64, -1_f64, 1_f64),
            Vector3::new( 1_f64, -1_f64, 1_f64),
        ];
        let expected = [
            Vector3::new( 1_f64 + shear_factor,  1_f64, 1_f64),
            Vector3::new(-1_f64 + shear_factor,  1_f64, 1_f64),
            Vector3::new(-1_f64 - shear_factor, -1_f64, 1_f64),
            Vector3::new( 1_f64 - shear_factor, -1_f64, 1_f64),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear2() {
        let shear_factor = 7_f64;
        let origin = Point2::new(-2_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_x());
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix3x3::new(
            1_f64,        0_f64, 0_f64,
            shear_factor, 1_f64, 0_f64,
            0_f64,        0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear3() {
        let shear_factor = 7_f64;
        let origin = Point2::new(-2_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_y());
        let normal = Unit::from_value(Vector2::unit_x());
        let matrix = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);
        let vectors = [
            Vector3::new( 1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64, -1_f64, 1_f64),
            Vector3::new( 1_f64, -1_f64, 1_f64),
        ];
        let expected = [
            Vector3::new( 1_f64,  1_f64 + 3_f64 * shear_factor, 1_f64),
            Vector3::new(-1_f64,  1_f64 + shear_factor,         1_f64),
            Vector3::new(-1_f64, -1_f64 + shear_factor,         1_f64),
            Vector3::new( 1_f64, -1_f64 + 3_f64 * shear_factor, 1_f64),
        ];
        let result = [
            matrix * vectors[0],
            matrix * vectors[1],
            matrix * vectors[2],
            matrix * vectors[3],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear4() {
        let shear_factor = 7_f64;
        let origin = Point2::new(-2_f64, 0_f64);
        let direction = Unit::from_value(Vector2::unit_y());
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix3x3::new(
            1_f64, shear_factor,              0_f64,
            0_f64, 1_f64,                     0_f64,
            0_f64, -origin[0] * shear_factor, 1_f64
        );
        let result = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear5() {
        let shear_factor = 7_f64;
        let origin = Point2::new(2_f64, 2_f64);
        let direction = Unit::from_value(Vector2::new(2_f64, 1_f64));
        let normal = Unit::from_value(Vector2::new(-1_f64, 2_f64));
        let matrix = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);
    
        let rotation_angle = Radians(f64::atan2(1_f64, 2_f64));
        assert_relative_eq!(rotation_angle.cos(), 2_f64 / f64::sqrt(5_f64), epsilon = 1e-10);
        assert_relative_eq!(rotation_angle.sin(), 1_f64 / f64::sqrt(5_f64), epsilon = 1e-10);

        let rotated_square = [
            Vector3::new( 1_f64 / f64::sqrt(5_f64),  3_f64 / f64::sqrt(5_f64) + 1_f64, 1_f64),
            Vector3::new(-3_f64 / f64::sqrt(5_f64),  1_f64 / f64::sqrt(5_f64) + 1_f64, 1_f64),
            Vector3::new(-1_f64 / f64::sqrt(5_f64), -3_f64 / f64::sqrt(5_f64) + 1_f64, 1_f64),
            Vector3::new( 3_f64 / f64::sqrt(5_f64), -1_f64 / f64::sqrt(5_f64) + 1_f64, 1_f64),
        ];
        let rotated_origin = Vector3::new(f64::sqrt(5_f64), 0_f64, 1_f64);
    
        let square = [
            Vector3::new( 1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64,  1_f64, 1_f64),
            Vector3::new(-1_f64, -1_f64, 1_f64),
            Vector3::new( 1_f64, -1_f64, 1_f64),
        ];
        let rotation = Matrix3x3::from_affine_angle(rotation_angle);
        let translation = Matrix3x3::from_affine_translation(&Vector2::new(0_f64, 1_f64));
        let result_rotated_square = [
            translation * rotation * square[0],
            translation * rotation * square[1],
            translation * rotation * square[2],
            translation * rotation * square[3],
        ];
    
        assert_relative_eq!(result_rotated_square[0], rotated_square[0], epsilon = 1e-10);
        assert_relative_eq!(result_rotated_square[1], rotated_square[1], epsilon = 1e-10);
        assert_relative_eq!(result_rotated_square[2], rotated_square[2], epsilon = 1e-10);
        assert_relative_eq!(result_rotated_square[3], rotated_square[3], epsilon = 1e-10);
    
        let result_rotated_translated_origin = translation * rotation * rotated_origin;
   
        assert_relative_eq!(result_rotated_translated_origin[0], origin[0], epsilon = 1e-10);
        assert_relative_eq!(result_rotated_translated_origin[1], origin[1], epsilon = 1e-10);
        assert_relative_eq!(result_rotated_translated_origin[2], 1_f64,     epsilon = 1e-10);

        let expected = [
            Vector3::new(
                 (1_f64 / f64::sqrt(5_f64)) + (2_f64 / f64::sqrt(5_f64)) * shear_factor,
                 (3_f64 / f64::sqrt(5_f64)) + (1_f64 / f64::sqrt(5_f64)) * shear_factor + 1_f64,
                 1_f64,
            ),
            Vector3::new(
                -(3_f64 / f64::sqrt(5_f64)) + (2_f64 / f64::sqrt(5_f64)) * shear_factor,
                 (1_f64 / f64::sqrt(5_f64))  + (1_f64 / f64::sqrt(5_f64)) * shear_factor + 1_f64,
                 1_f64,
            ),
            Vector3::new(
                -(1_f64 / f64::sqrt(5_f64)) - (2_f64 / f64::sqrt(5_f64)) * shear_factor,
                -(3_f64 / f64::sqrt(5_f64)) - (1_f64 / f64::sqrt(5_f64)) * shear_factor + 1_f64,
                 1_f64,
            ),
            Vector3::new(
                 (3_f64 / f64::sqrt(5_f64)) - (2_f64 / f64::sqrt(5_f64)) * shear_factor,
                -(1_f64 / f64::sqrt(5_f64)) - (1_f64 / f64::sqrt(5_f64)) * shear_factor + 1_f64,
                 1_f64,
            ),
        ];
        let result = [
            matrix * rotated_square[0],
            matrix * rotated_square[1],
            matrix * rotated_square[2],
            matrix * rotated_square[3],
        ];
    
        assert_relative_eq!(result[0], expected[0], epsilon = 1e-10);
        assert_relative_eq!(result[1], expected[1], epsilon = 1e-10);
        assert_relative_eq!(result[2], expected[2], epsilon = 1e-10);
        assert_relative_eq!(result[3], expected[3], epsilon = 1e-10);
    }

    #[test]
    fn test_from_affine_shear6() {
        let shear_factor = 7_f64;
        let origin = Point2::new(2_f64, 2_f64);
        let direction = Unit::from_value(Vector2::new(2_f64, 1_f64));
        let normal = Unit::from_value(Vector2::new(-1_f64, 2_f64));
        let expected = Matrix3x3::new(
             1_f64 - (2_f64 / 5_f64) * shear_factor, -(1_f64 / 5_f64) * shear_factor,         0_f64,
             (4_f64 / 5_f64) * shear_factor,          1_f64 + (2_f64 / 5_f64) * shear_factor, 0_f64,
            -(4_f64 / 5_f64) * shear_factor,         -(2_f64 / 5_f64) * shear_factor,         1_f64
        );
        let result = Matrix3x3::from_affine_shear(shear_factor, &origin, &direction, &normal);

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_x_axis1() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let bias = Point2::origin();
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix3x3::new(
            1_f64,  0_f64, 0_f64, 
            0_f64, -1_f64, 0_f64, 
            0_f64,  0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_x_axis2() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let bias = Point2::origin();
        let normal = Unit::from_value(-Vector2::unit_y());
        let expected = Matrix3x3::new(
            1_f64,  0_f64, 0_f64, 
            0_f64, -1_f64, 0_f64, 
            0_f64,  0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_y_axis1() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let bias = Point2::origin();
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix3x3::new(
            -1_f64, 0_f64, 0_f64, 
             0_f64, 1_f64, 0_f64, 
             0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_y_axis2() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let bias = Point2::origin();
        let normal = Unit::from_value(-Vector2::unit_x());
        let expected = Matrix3x3::new(
            -1_f64, 0_f64, 0_f64, 
             0_f64, 1_f64, 0_f64, 
             0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the 
    /// orientation of the line segment; there are two possible normal vectors 
    /// for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_from_plane1() {
        let bias = Point2::origin();
        let normal = Unit::from_value(
            Vector2::new(f64::sqrt(2_f64)/ 2_f64, -f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix3x3::new(
            0_f64, 1_f64, 0_f64, 
            1_f64, 0_f64, 0_f64, 
            0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
        
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the 
    /// orientation of the line segment; there are two possible normal vectors 
    /// for the line.
    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_from_plane2() {
        let bias = Point2::origin();
        let normal = Unit::from_value(
            Vector2::new(-f64::sqrt(2_f64)/ 2_f64, f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix3x3::new(
            0_f64, 1_f64, 0_f64, 
            1_f64, 0_f64, 0_f64, 
            0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
            
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// Construct an affine reflection matrix about the line `y = (1/2)x + 2`.
    /// This line does not cross the origin.
    #[test]
    fn test_from_affine_reflection_from_line_that_does_not_cross_origin1() {
        // We can always choose the y-intercept as the known point.
        let bias = Point2::new(0_f64, 2_f64);
        let normal = Unit::from_value(
            Vector2::new(-1_f64 / f64::sqrt(5_f64), 2_f64 / f64::sqrt(5_f64))
        );
        let matrix = Matrix3x3::from_affine_reflection(&normal, &bias);
        let vector = Vector3::new(1_f64, 0_f64, 1_f64);
        let expected = Vector3::new(-1_f64, 4_f64, 1_f64);
        let result = matrix * vector;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// Construct an affine reflection matrix about the line `y = (1/2)x + 2`.
    /// This line does not cross the origin.
    #[test]
    fn test_from_affine_reflection_from_line_that_does_not_cross_origin2() {
        // We can always choose the y-intercept as the known point.
        let bias = Point2::new(0_f64, 2_f64);
        let normal = Unit::from_value(
            Vector2::new(1_f64 / f64::sqrt(5_f64), -2_f64 / f64::sqrt(5_f64))
        );
        let matrix = Matrix3x3::from_affine_reflection(&normal, &bias);
        let vector = Vector3::new(1_f64, 0_f64, 1_f64);
        let expected = Vector3::new(-1_f64, 4_f64, 1_f64);
        let result = matrix * vector;

        assert_relative_eq!(result, expected, epsilon = 1e-8);        
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_xy_plane() {
        let normal = Unit::from_value(Vector3::unit_z());
        let expected = Matrix3x3::new(
            1_f64, 0_f64,  0_f64, 
            0_f64, 1_f64,  0_f64,  
            0_f64, 0_f64, -1_f64
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_zx_plane() {
        let normal = Unit::from_value(-Vector3::unit_y());
        let expected = Matrix3x3::new(
            1_f64,  0_f64, 0_f64, 
            0_f64, -1_f64, 0_f64,  
            0_f64,  0_f64, 1_f64
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_reflection_yz_plane() {
        let normal = Unit::from_value(Vector3::unit_x());
        let expected = Matrix3x3::new(
            -1_f64,  0_f64, 0_f64, 
             0_f64, 1_f64,  0_f64,  
             0_f64,  0_f64, 1_f64
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix3x3::from_angle_x(angle);
        let expected = unit_z;
        let result = matrix * unit_y;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix3x3::from_angle_y(angle);
        let expected = unit_x;
        let result = matrix * unit_z;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix3x3::from_angle_z(angle);
        let expected = unit_y;
        let result = matrix * unit_x;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_axis_angle() {
        let angle: Radians<f64> = Radians::full_turn_div_2();
        let axis = Unit::from_value(
            (1_f64 / f64::sqrt(2_f64)) * Vector3::new(1_f64, 1_f64, 0_f64)
        );
        let vector = Vector3::new(1_f64, 1_f64, -1_f64);
        let matrix = Matrix3x3::from_axis_angle(&axis, angle);
        let expected = Vector3::new(1_f64, 1_f64, 1_f64);
        let result = matrix * vector;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_affine_angle() {
        let matrix: Matrix3x3<f64> = Matrix3x3::from_affine_angle(Radians::full_turn_div_4());
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_y.extend(0_f64);
        let result = matrix * unit_x.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);

        let expected = -unit_x.extend(0_f64);
        let result = matrix * unit_y.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_point() {
        let distance = Vector2::new(3_i32, 7_i32);
        let matrix = Matrix3x3::from_affine_translation(&distance);
        let point = Vector3::new(0_i32, 0_i32, 1_i32);
        let expected = Vector3::new(3_i32, 7_i32, 1_i32);
        let result = matrix * point;

        assert_eq!(result, expected);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_vector() {
        let distance = Vector2::new(3_i32, 7_i32);
        let matrix = Matrix3x3::from_affine_translation(&distance);
        let vector = Vector3::zero();
        let expected = vector;
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_to_lh() {
        let direction = Vector3::new(1_f64, 1_f64, 1_f64);
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_to = Matrix3x3::look_to_lh(&direction, &up);
        let expected = unit_z;
        let result = look_to * direction.normalize();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_to_rh() {
        let direction = Vector3::new(1_f64, 1_f64, 1_f64).normalize();
        let up = Vector3::unit_y();
        let minus_unit_z = -Vector3::unit_z();
        let look_to = Matrix3x3::look_to_rh(&direction, &up);
        let expected = minus_unit_z;
        let result = look_to * direction;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_at_lh() {
        let eye = Point3::new(-1_f64, -1_f64, -1_f64);
        let target = Point3::origin();
        let direction = target - eye;
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_at = Matrix3x3::look_at_lh(&eye, &target, &up);
        let expected = unit_z;
        let result = look_at * direction.normalize();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_at_rh() {
        let eye = Point3::new(-1_f64, -1_f64, -1_f64);
        let target = Point3::origin();
        let direction = target - eye;
        let up = Vector3::unit_y();
        let minus_unit_z = -Vector3::unit_z();
        let look_at = Matrix3x3::look_at_rh(&eye, &target, &up);
        let expected = minus_unit_z;
        let result = look_at * direction.normalize();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_rotation_between() {
        let unit_x: Vector3<f64> = Vector3::unit_x();
        let unit_y: Vector3<f64> = Vector3::unit_y();
        let expected = Matrix3x3::new(
             0_f64, 1_f64, 0_f64, 
            -1_f64, 0_f64, 0_f64,
             0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::rotation_between(&unit_x, &unit_y).unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }
}

#[cfg(test)]
mod matrix4x4_tests {
    use cglinalg_trigonometry::{
        Angle,
        Radians,
        Degrees,
    };
    use cglinalg_core::{
        Vector3,
        Vector4,
        Normed,
        Matrix4x4,
        Unit,
        Point3,
    };
    use approx::{
        assert_relative_eq,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
        assert_eq!(matrix[2][0], 9_i32);
        assert_eq!(matrix[2][1], 10_i32);
        assert_eq!(matrix[2][2], 11_i32);
        assert_eq!(matrix[2][3], 12_i32);
        assert_eq!(matrix[3][0], 13_i32);
        assert_eq!(matrix[3][1], 14_i32);
        assert_eq!(matrix[3][2], 15_i32);
        assert_eq!(matrix[3][3], 16_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
        assert_eq!(matrix.c3r3, matrix[3][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[4][4], matrix[4][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_identity_matrix_times_identity_matrix_equals_identity_matrix() {
        let identity_matrix: Matrix4x4<f32> = Matrix4x4::identity();
        
        assert_eq!(identity_matrix * identity_matrix, identity_matrix);
    }

    #[test]
    fn test_zero_matrix_times_zero_matrix_equals_zero_matrix() {
        let zero_matrix: Matrix4x4<f32> = Matrix4x4::zero();

        assert_eq!(zero_matrix * zero_matrix, zero_matrix);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        // let expected = Matrix4x4::new(
        //     195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
        //     33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
        //     410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
        //     141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        // );
        let a_matrix_times_identity = a_matrix * Matrix4x4::identity();
        let b_matrix_times_identity = b_matrix * Matrix4x4::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        // let expected = Matrix4x4::new(
        //     195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
        //     33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
        //     410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
        //     141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix4x4::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix4x4::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix4x4::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix4x4::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        // let expected = Matrix4x4::new(
        //     195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
        //     33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
        //     410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
        //     141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        // );
        let zero_times_a_matrix = Matrix4x4::zero() * a_matrix;
        let zero_times_b_matrix = Matrix4x4::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix4x4::zero());
        assert_eq!(zero_times_b_matrix, Matrix4x4::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        // let expected = Matrix4x4::new(
        //     195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
        //     33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
        //     410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
        //     141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        // );
        let a_matrix_times_identity = a_matrix * Matrix4x4::identity();
        let identity_times_a_matrix = Matrix4x4::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix4x4::identity();
        let identity_times_b_matrix = Matrix4x4::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        // let expected = Matrix4x4::new(
        //     195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
        //     33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
        //     410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
        //     141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let a_matrix = Matrix4x4::new(
            80_f64,    23.43_f64,  43.56_f64, 6.74_f64, 
            426.1_f64, 23.57_f64,  27.61_f64, 13.90_f64,
            4.22_f64,  258.08_f64, 31.70_f64, 42.17_f64, 
            70_f64,    49_f64,     95_f64,    89.91_f64
        );
        let b_matrix = Matrix4x4::new(
            36.84_f64, 427.46_f64, 882.19_f64, 89.50_f64, 
            7.04_f64,  61.89_f64,  56.31_f64,  89_f64, 
            72_f64,    936.5_f64,  413.80_f64, 50.31_f64,  
            37.69_f64, 311.8_f64,  60.81_f64,  73.83_f64
        );
        let expected = Matrix4x4::new(
            195075.7478_f64, 242999.4886_f64, 49874.8440_f64, 51438.8929_f64,
            33402.1572_f64,  20517.1793_f64,  12255.4723_f64, 11284.3033_f64,
            410070.5860_f64, 133018.9590_f64, 46889.9950_f64, 35475.9481_f64,
            141297.8982_f64, 27543.7175_f64,  19192.1014_f64, 13790.4636_f64
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        // let expected = Matrix4x4::new(
        //     3943.4304_f64, 0_f64,           0_f64,           0_f64,
        //     0_f64,         356.8907901_f64, 0_f64,           0_f64,
        //     0_f64,         0_f64,           822.4719696_f64, 0_f64,
        //     0_f64,         0_f64,           0_f64,           238894.656314_f64
        // );
        let a_matrix_times_identity = a_matrix * Matrix4x4::identity();
        let b_matrix_times_identity = b_matrix * Matrix4x4::identity();

        assert_eq!(a_matrix_times_identity, a_matrix);
        assert_eq!(b_matrix_times_identity, b_matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        // let expected = Matrix4x4::new(
        //     3943.4304_f64, 0_f64,           0_f64,           0_f64,
        //     0_f64,         356.8907901_f64, 0_f64,           0_f64,
        //     0_f64,         0_f64,           822.4719696_f64, 0_f64,
        //     0_f64,         0_f64,           0_f64,           238894.656314_f64
        // );
        let a_matrix_times_zero_matrix = a_matrix * Matrix4x4::zero();
        let b_matrix_times_zero_matrix = b_matrix * Matrix4x4::zero();

        assert_eq!(a_matrix_times_zero_matrix, Matrix4x4::zero());
        assert_eq!(b_matrix_times_zero_matrix, Matrix4x4::zero());
    }

    #[test]
    fn test_zero_times_matrix_equals_zero2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        // let expected = Matrix4x4::new(
        //     3943.4304_f64, 0_f64,           0_f64,           0_f64,
        //     0_f64,         356.8907901_f64, 0_f64,           0_f64,
        //     0_f64,         0_f64,           822.4719696_f64, 0_f64,
        //     0_f64,         0_f64,           0_f64,           238894.656314_f64
        // );
        let zero_times_a_matrix = Matrix4x4::zero() * a_matrix;
        let zero_times_b_matrix = Matrix4x4::zero() * b_matrix;

        assert_eq!(zero_times_a_matrix, Matrix4x4::zero());
        assert_eq!(zero_times_b_matrix, Matrix4x4::zero());
    }

    #[test]
    fn test_matrix_times_identity_equals_identity_times_matrix2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        // let expected = Matrix4x4::new(
        //     3943.4304_f64, 0_f64,           0_f64,           0_f64,
        //     0_f64,         356.8907901_f64, 0_f64,           0_f64,
        //     0_f64,         0_f64,           822.4719696_f64, 0_f64,
        //     0_f64,         0_f64,           0_f64,           238894.656314_f64
        // );
        let a_matrix_times_identity = a_matrix * Matrix4x4::identity();
        let identity_times_a_matrix = Matrix4x4::identity() * a_matrix;
        let b_matrix_times_identity = b_matrix * Matrix4x4::identity();
        let identity_times_b_matrix = Matrix4x4::identity() * b_matrix;

        assert_eq!(a_matrix_times_identity, identity_times_a_matrix);
        assert_eq!(b_matrix_times_identity, identity_times_b_matrix);
    }

    #[test]
    fn test_matrix_transpose_transpose_equals_matrix2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        // let expected = Matrix4x4::new(
        //     3943.4304_f64, 0_f64,           0_f64,           0_f64,
        //     0_f64,         356.8907901_f64, 0_f64,           0_f64,
        //     0_f64,         0_f64,           822.4719696_f64, 0_f64,
        //     0_f64,         0_f64,           0_f64,           238894.656314_f64
        // );
        let a_matrix_transpose_transpose = a_matrix.transpose().transpose();
        let b_matrix_transpose_transpose = b_matrix.transpose().transpose();
            
        assert_eq!(a_matrix_transpose_transpose, a_matrix);
        assert_eq!(b_matrix_transpose_transpose, b_matrix);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let a_matrix = Matrix4x4::new(
            68.32_f64, 0_f64,      0_f64,     0_f64,
            0_f64,     37.397_f64, 0_f64,     0_f64,
            0_f64,     0_f64,      9.483_f64, 0_f64,
            0_f64,     0_f64,      0_f64,     887.710_f64
        );
        let b_matrix = Matrix4x4::new(
            57.72_f64, 0_f64,      0_f64,       0_f64, 
            0_f64,     9.5433_f64, 0_f64,       0_f64, 
            0_f64,     0_f64,      86.7312_f64, 0_f64,
            0_f64,     0_f64,      0_f64,       269.1134_f64
        );
        let expected = Matrix4x4::new(
            3943.4304_f64, 0_f64,           0_f64,           0_f64,
            0_f64,         356.8907901_f64, 0_f64,           0_f64,
            0_f64,         0_f64,           822.4719696_f64, 0_f64,
            0_f64,         0_f64,           0_f64,           238894.656314_f64
        );
        let result = a_matrix * b_matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix4x4::<f32>::identity();
        let identity_transpose = identity.transpose();
            
        assert_eq!(identity, identity_transpose);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector4::new(1_i32,  2_i32,  3_i32,  4_i32);
        let c1 = Vector4::new(5_i32,  6_i32,  7_i32,  8_i32);
        let c2 = Vector4::new(9_i32,  10_i32, 11_i32, 12_i32);
        let c3 = Vector4::new(13_i32, 14_i32, 15_i32, 16_i32);
        let columns = [c0, c1, c2, c3];
        let expected = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32, 
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let result = Matrix4x4::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector4::new(1_i32,  2_i32,  3_i32,  4_i32);
        let r1 = Vector4::new(5_i32,  6_i32,  7_i32,  8_i32);
        let r2 = Vector4::new(9_i32,  10_i32, 11_i32, 12_i32);
        let r3 = Vector4::new(13_i32, 14_i32, 15_i32, 16_i32);
        let rows = [r0, r1, r2, r3];
        let expected = Matrix4x4::new(
            1_i32, 5_i32, 9_i32,  13_i32,
            2_i32, 6_i32, 10_i32, 14_i32,
            3_i32, 7_i32, 11_i32, 15_i32,
            4_i32, 8_i32, 12_i32, 16_i32
        );
        let result = Matrix4x4::from_rows(&rows);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_identity_matrix4x4_translates_vector_along_vector() {
        let vector = Vector3::from((2_f64, 2_f64, 2_f64));
        let trans_matrix = Matrix4x4::from_affine_translation(&vector);
        let zero_vector4 = Vector4::from((0_f64, 0_f64, 0_f64, 1_f64));
        let zero_vector3 = Vector3::from((0_f64, 0_f64, 0_f64));

        let result = trans_matrix * zero_vector4;
        assert_eq!(result, (zero_vector3 + vector).extend(1_f64));
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169_f64;
        let identity = Matrix4x4::identity();
        let expected = Matrix4x4::new(
            c,     0_f64, 0_f64, 0_f64, 
            0_f64, c,     0_f64, 0_f64, 
            0_f64, 0_f64, c,     0_f64, 
            0_f64, 0_f64, 0_f64, c
        );

        assert_eq!(identity * c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169_f64;
        let identity = Matrix4x4::identity();
        let expected = Matrix4x4::new(
            1_f64 / c, 0_f64,     0_f64,     0_f64, 
            0_f64,     1_f64 / c, 0_f64,     0_f64, 
            0_f64,     0_f64,     1_f64 / c, 0_f64, 
            0_f64,     0_f64,     0_f64,     1_f64 / c
        );

        assert_eq!(identity / c, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix = Matrix4x4::zero();
        let matrix = Matrix4x4::new(
            36.84_f64,   427.46894_f64, 8827.1983_f64, 89.5049494_f64, 
            7.04217_f64, 61.891390_f64, 56.31_f64,     89_f64, 
            72_f64,      936.5_f64,     413.80_f64,    50.311160_f64,  
            37.6985_f64, 311.8_f64,     60.81_f64,     73.8393_f64
        );

        assert_eq!(matrix + zero_matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix = Matrix4x4::zero();
        let matrix = Matrix4x4::new(
            36.84_f64,   427.46894_f64, 8827.1983_f64, 89.5049494_f64, 
            7.04217_f64, 61.891390_f64, 56.31_f64,     89_f64, 
            72_f64,      936.5_f64,     413.80_f64,    50.311160_f64,  
            37.6985_f64, 311.8_f64,     60.81_f64,     73.8393_f64
        );

        assert_eq!(zero_matrix + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant() {
        // This matrix should have a zero determinant since it has two repeating columns.
        let matrix = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,  4_f64, 
            5_f64,  6_f64,  7_f64,  8_f64,
            5_f64,  6_f64,  7_f64,  8_f64, 
            9_f64,  10_f64, 11_f64, 12_f64
        );
        
        assert_eq!(matrix.determinant(), 0_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix = Matrix4x4::new(
            1_f64,  0_f64,  0_f64,  0_f64, 
            5_f64,  2_f64,  0_f64,  0_f64,
            5_f64,  5_f64,  3_f64,  0_f64, 
            5_f64,  5_f64,  5_f64,  4_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64 * 4_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix = Matrix4x4::new(
            1_f64,  5_f64,  5_f64,  5_f64, 
            0_f64,  2_f64,  5_f64,  5_f64,
            0_f64,  0_f64,  3_f64,  5_f64, 
            0_f64,  0_f64,  0_f64,  4_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64 * 4_f64);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_multiplication() {
        let result = (1_f64 / 32_f64) * Matrix4x4::new(
            7_f64, -1_f64, -1_f64, -1_f64,
           -1_f64,  7_f64, -1_f64, -1_f64,
           -1_f64, -1_f64,  7_f64, -1_f64,
           -1_f64, -1_f64, -1_f64,  7_f64
       );
       let expected = Matrix4x4::new(
           (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64,
           (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64,
           (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64,
           (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64
       );

       assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix4x4::new(
            5_f64, 1_f64, 1_f64, 1_f64, 
            1_f64, 5_f64, 1_f64, 1_f64,
            1_f64, 1_f64, 5_f64, 1_f64,
            1_f64, 1_f64, 1_f64, 5_f64, 
        );
        let expected = (1_f64 / 32_f64) * Matrix4x4::new(
             7_f64, -1_f64, -1_f64, -1_f64,
            -1_f64,  7_f64, -1_f64, -1_f64,
            -1_f64, -1_f64,  7_f64, -1_f64,
            -1_f64, -1_f64, -1_f64,  7_f64
        );
        let result = matrix.inverse().unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_identity_is_invertible() {
        assert!(Matrix4x4::<f64>::identity().is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix4x4<f64> = Matrix4x4::identity().inverse().unwrap();
        let expected: Matrix4x4<f64> = Matrix4x4::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix = 4_f64 * Matrix4x4::identity();
        let expected = (1_f64 / 4_f64) * Matrix4x4::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,   4_f64,
            5_f64,  60_f64, 7_f64,   8_f64,
            9_f64,  10_f64, 11_f64,  12_f64,
            13_f64, 14_f64, 150_f64, 16_f64
        );
        
        assert!(matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        // This matrix should not be invertible since it has two identical columns.
        let matrix = Matrix4x4::new(
            1_f64,  2_f64,   3_f64,  4_f64, 
            5_f64,  6_f64,   7_f64,  8_f64,
            5_f64,  6_f64,   7_f64,  8_f64, 
            9_f64,  10_f64,  11_f64, 12_f64
        );
        
        assert!(!matrix.is_invertible());
    }

    #[rustfmt::skip]
    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,  4_f64, 
            5_f64,  6_f64,  7_f64,  8_f64,
            5_f64,  6_f64,  7_f64,  8_f64, 
            9_f64,  10_f64, 11_f64, 12_f64
        );
        
        assert!(matrix.inverse().is_none());
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inversion2() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let result = matrix.inverse().unwrap();
        let expected = Matrix4x4::new(
             0.01146093272878252_f64,  -0.06212100841992658_f64, -0.02771783718075694_f64,    0.07986947998777854_f64,
            -0.00148039611514755_f64,   0.004464130960444646_f64, 0.003417891441120325_f64,  -0.005915083057511776_f64,
             0.001453087396607042_f64, -0.0009538600348427_f64,  -0.0005129477357421059_f64, -0.0002621470728476185_f64,
            -0.0007967195911958656_f64, 0.01365031989418242_f64,  0.0001408581712825875_f64, -0.002040325515611523_f64
        );

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix4x4::identity();

        assert_relative_eq!(matrix * matrix_inverse, identity, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let constant: f64 = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();

        assert_relative_eq!(matrix_transpose_inverse, matrix_inverse_transpose, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let matrix_inverse = matrix.inverse().unwrap();
        let identity = Matrix4x4::identity();
        
        assert_relative_eq!(matrix_inverse * matrix, identity, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix = Matrix4x4::new(
            36.84_f64,  427.468_f64, 882.198_f64, 89.504_f64, 
            7.042_f64,  61.891_f64,  56.31_f64,   89_f64, 
            72_f64,     936.5_f64,   413.80_f64,  50.311_f64,  
            37.698_f64, 311.8_f64,   60.81_f64,   73.839_f64
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32, 
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
        assert_eq!(matrix.c3r3, matrix[3][3]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,   4_i32, 
            5_i32,  6_i32,  7_i32,   8_i32, 
            9_i32,  10_i32, 11_i32,  12_i32,
            13_i32, 14_i32, 15_i32,  16_i32
        );
        result.swap_columns(3, 1);
        let expected = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            13_i32, 14_i32, 15_i32, 16_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            5_i32,  6_i32,  7_i32,  8_i32 
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32, 
            13_i32, 14_i32, 15_i32, 16_i32
        );
        result.swap_rows(3, 1);
        let expected = Matrix4x4::new(
            1_i32,  4_i32,  3_i32,  2_i32,
            5_i32,  8_i32,  7_i32,  6_i32,
            9_i32,  12_i32, 11_i32, 10_i32,
            13_i32, 16_i32, 15_i32, 14_i32
        );
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32, 
            13_i32, 14_i32, 15_i32, 16_i32
        );
        result.swap((2, 0), (1, 3));
        let expected = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  9_i32,
            8_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_scale() {
        let matrix = Matrix4x4::from_affine_scale(5_i32);
        let unit_w = Vector4::unit_w();
        let expected = Vector4::new(5_i32, 5_i32, 5_i32, 1_i32);
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
        assert_eq!(matrix * unit_w, unit_w);
    }

    #[test]
    fn test_from_affine_nonuniform_scale() {
        let matrix = Matrix4x4::from_affine_nonuniform_scale(&Vector3::new(5_i32, 7_i32, 11_i32));
        let unit_w = Vector4::unit_w();
        let expected = Vector4::new(5_i32, 7_i32, 11_i32, 1_i32);
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
        assert_eq!(matrix * unit_w, unit_w);
    }

    #[test]
    fn test_from_affine_shear_x() {
        let shear_x_with_y = 5_i32;
        let shear_x_with_z = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_x(shear_x_with_y, shear_x_with_z);
        let expected = Vector4::new(1_i32 + shear_x_with_y + shear_x_with_z, 1_i32, 1_i32, 1_i32);
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x_does_not_change_last_coordinate() {
        let shear_x_with_y = 5_i32;
        let shear_x_with_z = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_x(shear_x_with_y, shear_x_with_z);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y() {
        let shear_y_with_x = 3_i32;
        let shear_y_with_z = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_y(shear_y_with_x, shear_y_with_z);
        let expected = Vector4::new(1_i32, 1_i32 + shear_y_with_x + shear_y_with_z, 1_i32, 1_i32);
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y_does_not_change_last_coordinate() {
        let shear_y_with_x = 3_i32;
        let shear_y_with_z = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_y(shear_y_with_x, shear_y_with_z);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_z() {
        let shear_z_with_x = 3_i32;
        let shear_z_with_y = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_z(shear_z_with_x, shear_z_with_y);
        let expected = Vector4::new(1_i32, 1_i32, 1_i32 + shear_z_with_x + shear_z_with_y, 1_i32);
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_z_does_not_change_last_coordinate() {
        let shear_z_with_x = 3_i32;
        let shear_z_with_y = 11_i32;
        let matrix = Matrix4x4::from_affine_shear_z(shear_z_with_x, shear_z_with_y);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear() {
        let shear_x_with_y = 2_i32;
        let shear_x_with_z = 4_i32;
        let shear_y_with_x = 8_i32;
        let shear_y_with_z = 7_i32;
        let shear_z_with_x = 3_i32;
        let shear_z_with_y = 11_i32;
        let matrix = Matrix4x4::from_affine_shear(
            shear_x_with_y, shear_x_with_z, shear_y_with_x, shear_y_with_z, shear_z_with_x, shear_z_with_y
        );
        let expected = Vector4::new(
            1_i32 + shear_x_with_y + shear_x_with_z, 
            1_i32 + shear_y_with_x + shear_y_with_z, 
            1_i32 + shear_z_with_x + shear_z_with_y, 
            1_i32
        );
        let result = matrix * Vector4::new(1_i32, 1_i32, 1_i32, 1_i32);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_does_not_change_last_coordinate() {
        let shear_x_with_y = 2_i32;
        let shear_x_with_z = 4_i32;
        let shear_y_with_x = 8_i32;
        let shear_y_with_z = 7_i32;
        let shear_z_with_x = 3_i32;
        let shear_z_with_y = 11_i32;
        let matrix = Matrix4x4::from_affine_shear(
            shear_x_with_y, shear_x_with_z, shear_y_with_x, shear_y_with_z, shear_z_with_x, shear_z_with_y
        );
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_xy_plane() {
        let bias = Point3::origin();
        let normal = Unit::from_value(Vector3::unit_z());
        let expected = Matrix4x4::new(
            1_f64, 0_f64,  0_f64, 0_f64,
            0_f64, 1_f64,  0_f64, 0_f64,
            0_f64, 0_f64, -1_f64, 0_f64,
            0_f64, 0_f64,  0_f64, 1_f64
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_zx_plane() {
        let bias = Point3::origin();
        let normal = Unit::from_value(-Vector3::unit_y());
        let expected = Matrix4x4::new(
            1_f64,  0_f64, 0_f64, 0_f64,
            0_f64, -1_f64, 0_f64, 0_f64,
            0_f64,  0_f64, 1_f64, 0_f64,
            0_f64,  0_f64, 0_f64, 1_f64
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_affine_reflection_yz_plane() {
        let bias = Point3::origin();
        let normal = Unit::from_value(Vector3::unit_x());
        let expected = Matrix4x4::new(
            -1_f64,  0_f64, 0_f64,  0_f64,
             0_f64,  1_f64, 0_f64,  0_f64,
             0_f64,  0_f64, 1_f64,  0_f64,
             0_f64,  0_f64, 0_f64,  1_f64
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_plane1() {
        // The plane `z = 1`.
        let bias = Point3::new(0_f64, 0_f64, 1_f64);
        let normal = Unit::from_value(Vector3::new(0_f64, 0_f64, 1_f64));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(1_f64, 1_f64, 0.5_f64, 1_f64);
        let expected = Vector4::new(1_f64,1_f64,1.5_f64, 1_f64);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_plane2() {
        // The plane `x = -1`.
        let bias = Point3::new(-1_f64, 0_f64, 0_f64);
        let normal = Unit::from_value(Vector3::new(1_f64, 0_f64, 0_f64));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(-2_f64, 1_f64, 1_f64, 1_f64);
        let expected = Vector4::new(0_f64,1_f64,1_f64, 1_f64);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_plane3() {
        // The plane `y = 1`.
        let bias = Point3::new(0_f64, 1_f64, 0_f64);
        let normal = Unit::from_value(Vector3::new(0_f64, 1_f64, 0_f64));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(0_f64, 0_f64, 0_f64, 1_f64);
        let expected = Vector4::new(0_f64,2_f64, 0_f64, 1_f64);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix4x4::from_affine_angle_x(angle);
        let expected = unit_z.extend(0_f64);
        let result = matrix * unit_y.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix4x4::from_affine_angle_y(angle);
        let expected = unit_x.extend(0_f64);
        let result = matrix * unit_z.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix4x4::from_affine_angle_z(angle);
        let expected = unit_y.extend(0_f64);
        let result = matrix * unit_x.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_affine_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix4x4::from_affine_angle_x(angle);
        let expected = unit_z.extend(0_f64);
        let result = matrix * unit_y.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_affine_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix4x4::from_affine_angle_y(angle);
        let expected = unit_x.extend(0_f64);
        let result = matrix * unit_z.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_affine_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix4x4::from_affine_angle_z(angle);
        let expected = unit_y.extend(0_f64);
        let result = matrix * unit_x.extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_from_affine_axis_angle() {
        let angle: Radians<f64> = Radians::full_turn_div_2();
        let axis = Unit::from_value(
            (1_f64 / f64::sqrt(2_f64)) * Vector3::new(1_f64, 1_f64, 0_f64)
        );
        let vector = Vector4::new(1_f64, 1_f64, -1_f64, 0_f64);
        let matrix = Matrix4x4::from_affine_axis_angle(&axis, angle);
        let expected = Vector4::new(1_f64, 1_f64, 1_f64, 0_f64);
        let result = matrix * vector;

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_point() {
        let distance = Vector3::new(3_i32, 7_i32, 11_i32);
        let matrix = Matrix4x4::from_affine_translation(&distance);
        let point = Vector4::new(0_i32, 0_i32, 0_i32, 1_i32);
        let expected = Vector4::new(3_i32, 7_i32, 11_i32, 1_i32);
        let result = matrix * point;

        assert_eq!(result, expected);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_vector() {
        let distance = Vector3::new(3_i32, 7_i32, 11_i32);
        let matrix = Matrix4x4::from_affine_translation(&distance);
        let vector = Vector4::new(0_i32, 0_i32, 0_i32, 0_i32);
        let expected = vector;
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_orthographic() {
        let left = -4_f64;
        let right = 4_f64;
        let bottom = -2_f64;
        let top = 2_f64;
        let near = 1_f64;
        let far = 100_f64;
        let expected = Matrix4x4::new(
            1_f64 / 4_f64,  0_f64,          0_f64,            0_f64,
            0_f64,          1_f64 / 2_f64,  0_f64,            0_f64,
            0_f64,          0_f64,         -2_f64 / 99_f64,   0_f64,
            0_f64,          0_f64,         -101_f64 / 99_f64, 1_f64
        );
        let result = Matrix4x4::from_orthographic(left, right, bottom, top, near, far);
    
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_perspective_fov() {
        let vfov = Degrees(72_f32);
        let aspect_ratio = 800_f32 / 600_f32;
        let near = 0.1_f32;
        let far = 100_f32;
        let expected = Matrix4x4::new(
            1.0322863_f32, 0_f32,          0_f32,         0_f32, 
            0_f32,         1.3763818_f32,  0_f32,         0_f32, 
            0_f32,         0_f32,         -1.002002_f32, -1_f32, 
            0_f32,         0_f32,         -0.2002002_f32, 0_f32
        );
        let result = Matrix4x4::from_perspective_fov(vfov, aspect_ratio, near, far);
    
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[rustfmt::skip]
    #[test]
    fn test_from_perspective() {
        let left = -4_f64;
        let right = 4_f64;
        let bottom = -2_f64;
        let top = 3_f64;
        let near = 1_f64;
        let far = 100_f64;
        let expected = Matrix4x4::new(
            1_f64 / 4_f64, 0_f64,          0_f64,             0_f64,
            0_f64,         2_f64 / 5_f64,  0_f64,             0_f64,
            0_f64,         1_f64 / 5_f64, -101_f64 / 99_f64, -1_f64,
            0_f64,         0_f64,         -200_f64 / 99_f64,  0_f64
        );
        let result = Matrix4x4::from_perspective(left, right, bottom, top, near, far);
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_rh_at_origin() {
        let eye = Point3::new(0_f64, 0_f64, 0_f64);
        let target = Point3::new(1_f64, 1_f64, 1_f64);
        let up = Vector3::unit_y();
        let minus_unit_z = -Vector3::unit_z();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = minus_unit_z.extend(0_f64);
        let result = look_at * direction.normalize().extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_at_lh_at_origin() {
        let eye = Point3::new(0_f64, 0_f64, 0_f64);
        let target = Point3::new(1_f64, 1_f64, 1_f64);
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = unit_z.extend(0_f64);
        let result = look_at * direction.normalize().extend(0_f64);

        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_at_lh_no_displacement_or_rotation() {
        let eye = Point3::new(0_f64, 0_f64, 0_f64);
        let target = Point3::new(0_f64, 0_f64, 1_f64);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = Vector4::new(0_f64, 0_f64, 1_f64, 0_f64);
        let result = look_at * direction.normalize().extend(0_f64);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_rh_no_displacement_or_rotation() {
        let eye = Point3::new(0_f64, 0_f64, 0_f64);
        let target = Point3::new(0_f64, 0_f64, 1_f64);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let expected = Vector4::new(0_f64, 0_f64, 0_f64, 1_f64);
        let result = look_at * eye.to_homogeneous();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_lh_eye_to_origin() {
        let eye = Point3::new(-1_f64, -1_f64, -1_f64);
        let target = Point3::new(1_f64, 1_f64, 1_f64);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let expected = Vector4::unit_w();
        let result = look_at * eye.to_homogeneous();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_rh_eye_to_origin() {
        let eye = Point3::new(-1_f64, -1_f64, -1_f64);
        let target = Point3::new(1_f64, 1_f64, 1_f64);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let expected = Vector4::unit_w();
        let result = look_at * eye.to_homogeneous();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_to_lh1() {
        let eye = Point3::new(1_f64, 1_f64, 1_f64);
        let direction = (eye - Point3::origin()).normalize();
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z().to_homogeneous();
        let look_to = Matrix4x4::look_to_lh(&eye, &direction, &up);
        let expected = unit_z;
        let result = look_to * direction.normalize().to_homogeneous();
    
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }

    #[test]
    fn test_look_to_lh2() {
        let eye = Point3::new(1_f64, 1_f64, 1_f64);
        let direction = (eye - Point3::origin()).normalize();
        let up = Vector3::unit_y();
        let minus_unit_z = (-Vector3::unit_z()).to_homogeneous();
        let look_to = Matrix4x4::look_to_lh(&eye, &direction, &up);
        let expected = minus_unit_z;
        let result = look_to * (-direction).normalize().to_homogeneous();
    
        assert_relative_eq!(result, expected, epsilon = 1e-8);
    }
}


#[cfg(test)]
mod matrix1x2_tests {
    use cglinalg_core::{
        Vector1,
        Vector2,
        Matrix1x2,
        Matrix2x2,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[0][1], matrix[0][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[2][1], matrix[2][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix1x2::new(2_i32, 3_i32);
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix1x2::new(33_i32, 54_i32);
        let zero_matrix2x2 = Matrix2x2::zero();
        let zero_matrix1x2 = Matrix1x2::zero();

        assert_eq!(matrix * zero_matrix2x2, zero_matrix1x2);
    }

    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix1x2::new(33_i32, 54_i32);
        let zero = 0_i32;
        let zero_matrix1x2 = Matrix1x2::zero();

        assert_eq!(zero * matrix, zero_matrix1x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x2 = Matrix1x2::new(2_i32, 3_i32);
        let matrix2x2 = Matrix2x2::new(
            1_i32, 2_i32, 
            3_i32, 4_i32
        );
        let expected = Matrix1x2::new(8_i32, 18_i32);
        let result = matrix1x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x2 = Matrix1x2::new(4_i32, 5_i32);
        let vector = Vector2::new(9_i32, 6_i32);
        let expected = Vector1::new(66_i32);
        let result = matrix1x2 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x2 = Matrix1x2::new(1_i32, 2_i32);
        let scalar = 13_i32;
        let expected = Matrix1x2::new(13_i32, 26_i32);
        let result = matrix1x2 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x2 = Matrix1x2::new(1_i32, 2_i32);
        let scalar = 13_i32;
        let expected = Matrix1x2::new(13_i32, 26_i32);
        let result = scalar * matrix1x2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix1x2 = Matrix1x2::zero();
        let matrix = Matrix1x2::new(3684_i32, 42746_i32);

        assert_eq!(matrix + zero_matrix1x2, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix1x2 = Matrix1x2::zero();
        let matrix = Matrix1x2::new(3684_i32, 42746_i32);

        assert_eq!(zero_matrix1x2 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x2::new(23_i32, 76_i32);
        let matrix2 = Matrix1x2::new(1_i32, 5_i32);
        let expected = Matrix1x2::new(24_i32, 81_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x2::new(3_i32, 6_i32);
        let matrix2 = Matrix1x2::new(1_i32, 15_i32);
        let expected = Matrix1x2::new(2_i32, -9_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x2::new(3_i32, 6_i32);
        let zero_matrix1x2 = Matrix1x2::zero();

        assert_eq!(matrix - matrix, zero_matrix1x2);
    }
}

#[cfg(test)]
mod matrix1x3_tests {
    use cglinalg_core::{
        Vector1,
        Vector3,
        Matrix1x3,
        Matrix3x3,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
        assert_eq!(matrix[2][0], 3_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[0][1], matrix[0][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[3][1], matrix[3][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix1x3::new(2_i32, 3_i32, 4_i32);
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix1x3::new(33_i32, 54_i32, 19_i32);
        let zero_matrix3x3 = Matrix3x3::zero();
        let zero_matrix1x3 = Matrix1x3::zero();

        assert_eq!(matrix * zero_matrix3x3, zero_matrix1x3);
    }

    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix1x3::new(33_i32, 54_i32, 19_i32);
        let zero = 0_i32;
        let zero_matrix1x3 = Matrix1x3::zero();

        assert_eq!(zero * matrix, zero_matrix1x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x3 = Matrix1x3::new(2_i32, 3_i32, 4_i32);
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix1x3::new(20_i32, 47_i32, 74_i32);
        let result = matrix1x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x3 = Matrix1x3::new(4_i32, 5_i32, 6_i32);
        let vector = Vector3::new(9_i32, 6_i32, -12_i32);
        let expected = Vector1::new(-6_i32);
        let result = matrix1x3 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x3 = Matrix1x3::new(1_i32, 2_i32, 3_i32);
        let scalar = 13_i32;
        let expected = Matrix1x3::new(13_i32, 26_i32, 39_i32);
        let result = matrix1x3 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x3 = Matrix1x3::new(1_i32, 2_i32, 3_i32);
        let scalar = 13_i32;
        let expected = Matrix1x3::new(13_i32, 26_i32, 39_i32);
        let result = scalar * matrix1x3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix1x3 = Matrix1x3::zero();
        let matrix = Matrix1x3::new(3684_i32, 42746_i32, 345_i32);

        assert_eq!(matrix + zero_matrix1x3, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix1x3 = Matrix1x3::zero();
        let matrix = Matrix1x3::new(3684_i32, 42746_i32, 345_i32);

        assert_eq!(zero_matrix1x3 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x3::new(23_i32, 76_i32, 89_i32);
        let matrix2 = Matrix1x3::new(1_i32, 5_i32, 9_i32);
        let expected = Matrix1x3::new(24_i32, 81_i32, 98_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x3::new(3_i32, 6_i32, 9_i32);
        let matrix2 = Matrix1x3::new(1_i32, 15_i32, 29_i32);
        let expected = Matrix1x3::new(2_i32, -9_i32, -20_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x3::new(3_i32, 6_i32, 9_i32);
        let zero_matrix1x3 = Matrix1x3::zero();

        assert_eq!(matrix - matrix, zero_matrix1x3);
    }
}

#[cfg(test)]
mod matrix1x4_tests {
    use cglinalg_core::{
        Vector1,
        Vector4,
        Matrix1x4,
        Matrix4x4,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
        assert_eq!(matrix[2][0], 3_i32);
        assert_eq!(matrix[3][0], 4_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][1], matrix[0][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[4][1], matrix[4][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix1x4::new(2_i32, 3_i32, 4_i32, 5_i32);
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix1x4::new(33_i32, 54_i32, 19_i32, 5_i32);
        let zero_matrix4x4 = Matrix4x4::zero();
        let zero_matrix1x4 = Matrix1x4::zero();

        assert_eq!(matrix * zero_matrix4x4, zero_matrix1x4);
    }

    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix1x4::new(33_i32, 54_i32, 19_i32, 5_i32);
        let zero = 0_i32;
        let zero_matrix1x4 = Matrix1x4::zero();

        assert_eq!(zero * matrix, zero_matrix1x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x4 = Matrix1x4::new(2_i32, 3_i32, 4_i32, 5_i32);
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix1x4::new(40_i32, 96_i32, 152_i32, 208_i32);
        let result = matrix1x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x4 = Matrix1x4::new(4_i32, 5_i32, 6_i32, 7_i32);
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -72_i32);
        let expected = Vector1::new(-510_i32);
        let result = matrix1x4 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x4 = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let scalar = 13_i32;
        let expected = Matrix1x4::new(13_i32, 26_i32, 39_i32, 52_i32);
        let result = matrix1x4 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x4 = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let scalar = 13_i32;
        let expected = Matrix1x4::new(13_i32, 26_i32, 39_i32, 52_i32);
        let result = scalar * matrix1x4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix1x4 = Matrix1x4::zero();
        let matrix = Matrix1x4::new(3684_i32, 42746_i32, 345_i32, 546_i32);

        assert_eq!(matrix + zero_matrix1x4, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix1x4 = Matrix1x4::zero();
        let matrix = Matrix1x4::new(3684_i32, 42746_i32, 345_i32, 546_i32);

        assert_eq!(zero_matrix1x4 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x4::new(23_i32, 76_i32, 89_i32, 34_i32);
        let matrix2 = Matrix1x4::new(1_i32, 5_i32, 9_i32, 13_i32);
        let expected = Matrix1x4::new(24_i32, 81_i32, 98_i32, 47_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x4::new(3_i32, 6_i32, 9_i32, 12_i32);
        let matrix2 = Matrix1x4::new(1_i32, 15_i32, 29_i32, 6_i32);
        let expected = Matrix1x4::new(2_i32, -9_i32, -20_i32, 6_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x4::new(3_i32, 6_i32, 9_i32, 12_i32);
        let zero_matrix1x4 = Matrix1x4::zero();

        assert_eq!(matrix - matrix, zero_matrix1x4);
    }
}



#[cfg(test)]
mod matrix2x3_tests {
    use cglinalg_core::{
        Vector3,
        Vector2,
        Matrix2x2,
        Matrix2x3,
        Matrix3x2,
        Matrix3x3,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
        assert_eq!(matrix[2][0], 5_i32);
        assert_eq!(matrix[2][1], 6_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[3][2], matrix[3][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32,
            6_i32, 7_i32
        );
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix2x3::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32
        );
        let zero_matrix3x3 = Matrix3x3::zero();
        let zero_matrix2x3 = Matrix2x3::zero();

        assert_eq!(matrix * zero_matrix3x3, zero_matrix2x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix2x3::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            234_i32, 98_i32
        );
        let zero = 0_i32;
        let zero_matrix2x3 = Matrix2x3::zero();

        assert_eq!(zero * matrix, zero_matrix2x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix2x3 = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32
        );
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 8_i32,
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix2x3::new(
            28_i32,  34_i32, 
            76_i32,  93_i32, 
            100_i32, 124_i32    
        );
        let result = matrix2x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix2x3 = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32
        );
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 8_i32
        );
        let expected = Matrix2x2::new(
            28_i32,  34_i32, 
            76_i32,  93_i32
        );
        let result = matrix2x3 * matrix3x2;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix2x3 = Matrix2x3::new(
            4_i32, 5_i32, 
            6_i32, 7_i32, 
            8_i32, 9_i32
        );
        let vector = Vector3::new(9_i32, 6_i32, -12_i32);
        let expected = Vector2::new(-24_i32, -21_i32);
        let result = matrix2x3 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x3::new(
            13_i32, 26_i32,
            39_i32, 52_i32,
            65_i32, 91_i32
        );
        let result = matrix2x3 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x3::new(
            13_i32, 26_i32,
            39_i32, 52_i32,
            65_i32, 91_i32
        );
        let result = scalar * matrix2x3;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix2x3 = Matrix2x3::zero();
        let matrix = Matrix2x3::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32
        );

        assert_eq!(matrix + zero_matrix2x3, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix2x3 = Matrix2x3::zero();
        let matrix = Matrix2x3::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32
        );

        assert_eq!(zero_matrix2x3 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix2x3::new(
            23_i32,  76_i32, 
            89_i32,  34_i32,
            324_i32, 75_i32
        );
        let matrix2 = Matrix2x3::new(
            1_i32,  5_i32, 
            9_i32,  13_i32,
            17_i32, 21_i32
        );
        let expected = Matrix2x3::new(
            24_i32,  81_i32, 
            98_i32,  47_i32,
            341_i32, 96_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix2x3::new(
            3_i32, 6_i32, 
            9_i32, 12_i32,
            15_i32, 18_i32
            
        );
        let matrix2 = Matrix2x3::new(
            1_i32,   15_i32, 
            29_i32,  6_i32,
            234_i32, 93_i32,
        );
        let expected = Matrix2x3::new(
             2_i32,  -9_i32, 
            -20_i32,  6_i32,
            -219_i32, -75_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix2x3::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32
        );
        let zero_matrix2x3 = Matrix2x3::zero();

        assert_eq!(matrix - matrix, zero_matrix2x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );
        let expected = Matrix3x2::new(
            1_i32, 3_i32, 5_i32,
            2_i32, 4_i32, 6_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector2::new(1_i32, 2_i32);
        let c1 = Vector2::new(3_i32, 4_i32);
        let c2 = Vector2::new(5_i32, 6_i32);
        let columns = [c0, c1, c2];
        let expected = Matrix2x3::new(
            1_i32, 2_i32,
            3_i32, 4_i32,
            5_i32, 6_i32
        );
        let result = Matrix2x3::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector3::new(1_i32, 2_i32, 3_i32);
        let r1 = Vector3::new(4_i32, 5_i32, 6_i32);
        let rows = [r0, r1];
        let expected = Matrix2x3::new(
            1_i32, 4_i32,
            2_i32, 5_i32,
            3_i32, 6_i32
        );
        let result = Matrix2x3::from_rows(&rows);

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix3x2_tests {
    use cglinalg_core::{
        Vector3,
        Vector2,
        Matrix2x2,
        Matrix2x3,
        Matrix3x2,
        Matrix3x3,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[2][3], matrix[2][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix3x2::new(
            33_i32, 54_i32,  19_i32,
            5_i32,  793_i32, 23_i32
        );
        let zero_matrix2x2 = Matrix2x2::zero();
        let zero_matrix3x2 = Matrix3x2::zero();

        assert_eq!(matrix * zero_matrix2x2, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix3x2::new(
            33_i32, 54_i32,  19_i32,
            5_i32,  234_i32, 98_i32
        );
        let zero = 0_i32;
        let zero_matrix3x2 = Matrix3x2::zero();

        assert_eq!(zero * matrix, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix3x2 = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let matrix2x2 = Matrix2x2::new(
            1_i32, 2_i32, 
            4_i32, 5_i32,
        );
        let expected = Matrix3x2::new(
            12_i32, 15_i32, 18_i32,  
            33_i32, 42_i32, 51_i32    
        );
        let result = matrix3x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix3x2 = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 8_i32
        );
        let expected = Matrix3x3::new(
            12_i32, 15_i32, 18_i32,
            26_i32, 33_i32, 40_i32,
            50_i32, 63_i32, 76_i32
        );
        let result = matrix3x2 * matrix2x3;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix3x2 = Matrix3x2::new(
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let vector = Vector2::new(9_i32, -6_i32);
        let expected = Vector3::new(-6_i32, -3_i32, 0_i32);
        let result = matrix3x2 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x2::new(
            13_i32, 26_i32, 39_i32, 
            52_i32, 65_i32, 91_i32
        );
        let result = matrix3x2 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x2::new(
            13_i32, 26_i32, 39_i32, 
            52_i32, 65_i32, 91_i32
        );
        let result = scalar * matrix3x2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix3x2 = Matrix3x2::zero();
        let matrix = Matrix3x2::new(
            3684_i32, 42746_i32, 345_i32, 
            546_i32,  76_i32,    167_i32
        );

        assert_eq!(matrix + zero_matrix3x2, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix3x2 = Matrix3x2::zero();
        let matrix = Matrix3x2::new(
            3684_i32, 42746_i32, 345_i32, 
            546_i32,  76_i32,    167_i32
        );

        assert_eq!(zero_matrix3x2 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix3x2::new(
            23_i32, 76_i32,  89_i32,  
            34_i32, 324_i32, 75_i32
        );
        let matrix2 = Matrix3x2::new(
            1_i32,  5_i32,  9_i32,  
            13_i32, 17_i32, 21_i32
        );
        let expected = Matrix3x2::new(
            24_i32, 81_i32, 98_i32,
            47_i32, 341_i32, 96_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix3x2::new(
            3_i32,  6_i32,  9_i32, 
            12_i32, 15_i32, 18_i32
        );
        let matrix2 = Matrix3x2::new(
            1_i32, 15_i32,  29_i32,  
            6_i32, 234_i32, 93_i32,
        );
        let expected = Matrix3x2::new(
             2_i32, -9_i32,   -20_i32, 
             6_i32, -219_i32, -75_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix3x2::new(
            3_i32,  6_i32,  9_i32,
            12_i32, 15_i32, 18_i32
        );
        let zero_matrix3x2 = Matrix3x2::zero();

        assert_eq!(matrix - matrix, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix3x2::new(
            1_i32, 3_i32, 5_i32,
            2_i32, 4_i32, 6_i32
        );
        let expected = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector3::new(1_i32, 2_i32, 3_i32);
        let c1 = Vector3::new(4_i32, 5_i32, 6_i32);
        let columns = [c0, c1];
        let expected = Matrix3x2::new(
            1_i32, 2_i32, 3_i32,
            4_i32, 5_i32, 6_i32
        );
        let result = Matrix3x2::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector2::new(1_i32, 2_i32);
        let r1 = Vector2::new(3_i32, 4_i32);
        let r2 = Vector2::new(5_i32, 6_i32);
        let rows = [r0, r1, r2];
        let expected = Matrix3x2::new(
            1_i32, 3_i32, 5_i32,
            2_i32, 4_i32, 6_i32
        );
        let result = Matrix3x2::from_rows(&rows);

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix2x4_tests {
    use cglinalg_core::{
        Vector4,
        Vector2,
        Matrix2x4,
        Matrix4x2,
        Matrix4x4,
        Matrix2x2,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
        assert_eq!(matrix[2][0], 5_i32);
        assert_eq!(matrix[2][1], 6_i32);
        assert_eq!(matrix[3][0], 7_i32);
        assert_eq!(matrix[3][1], 8_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[4][2], matrix[4][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix2x4::new(
            2_i32, 3_i32, 
            4_i32, 5_i32,
            6_i32, 7_i32,
            8_i32, 9_i32
        );
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32,
            49_i32,  11_i32
        );
        let zero_matrix4x4 = Matrix4x4::zero();
        let zero_matrix2x4 = Matrix2x4::zero();

        assert_eq!(matrix * zero_matrix4x4, zero_matrix2x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_matrix_times_matrix_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32,
            49_i32,  11_i32
        );
        let zero_matrix2x2 = Matrix2x2::zero();
        let zero_matrix2x4 = Matrix2x4::zero();

        assert_eq!(zero_matrix2x2 * matrix, zero_matrix2x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            234_i32, 98_i32,
            64_i32,  28_i32
        );
        let zero = 0_i32;
        let zero_matrix2x4 = Matrix2x4::zero();

        assert_eq!(zero * matrix, zero_matrix2x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix2x4 = Matrix2x4::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32,
            8_i32, 9_i32
        );
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix2x4::new(
            60_i32,  70_i32,
            140_i32, 166_i32,
            220_i32, 262_i32,
            300_i32, 358_i32
        );
        let result = matrix2x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix2x4 = Matrix2x4::new(
            4_i32,  5_i32, 
            6_i32,  7_i32, 
            8_i32,  9_i32, 
            10_i32, 11_i32
        );
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -24_i32);
        let expected = Vector2::new(-264_i32, -285_i32);
        let result = matrix2x4 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix2x4 = Matrix2x4::new(
            2_i32, 3_i32,
            4_i32, 5_i32, 
            6_i32, 7_i32, 
            8_i32, 9_i32,
        );
        let vector = Vector4::new(9_i32, -6_i32, 12_i32, 4_i32);
        let expected = Vector2::new(98_i32, 117_i32);
        let result = matrix2x4 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32,
            8_i32, 9_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x4::new(
            13_i32,  26_i32,
            39_i32,  52_i32,
            65_i32,  91_i32,
            104_i32, 117_i32
        );
        let result = matrix2x4 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32,
            8_i32, 9_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x4::new(
            13_i32,  26_i32,
            39_i32,  52_i32,
            65_i32,  91_i32,
            104_i32, 117_i32
        );
        let result = scalar * matrix2x4;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix2x4 = Matrix2x4::zero();
        let matrix = Matrix2x4::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32,
            415_i32,  251_i32
        );

        assert_eq!(matrix + zero_matrix2x4, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix2x4 = Matrix2x4::zero();
        let matrix = Matrix2x4::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32,
            415_i32,  251_i32
        );

        assert_eq!(zero_matrix2x4 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix2x4::new(
            23_i32,  76_i32, 
            89_i32,  34_i32,
            324_i32, 75_i32,
            614_i32, 15_i32
        );
        let matrix2 = Matrix2x4::new(
            1_i32,  5_i32, 
            9_i32,  13_i32,
            17_i32, 21_i32,
            87_i32, 41_i32
        );
        let expected = Matrix2x4::new(
            24_i32,  81_i32, 
            98_i32,  47_i32,
            341_i32, 96_i32,
            701_i32, 56_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix2x4::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32,
            21_i32, 24_i32
        );
        let matrix2 = Matrix2x4::new(
            1_i32,   15_i32, 
            29_i32,  6_i32,
            234_i32, 93_i32,
            93_i32,  7_i32
        );
        let expected = Matrix2x4::new(
             2_i32,   -9_i32, 
            -20_i32,   6_i32,
            -219_i32, -75_i32,
            -72_i32,   17_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix2x4::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32,
            21_i32, 24_i32
        );
        let zero_matrix2x4 = Matrix2x4::zero();

        assert_eq!(matrix - matrix, zero_matrix2x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32,
            7_i32, 8_i32
        );
        let expected = Matrix4x2::new(
            1_i32, 3_i32, 5_i32, 7_i32, 
            2_i32, 4_i32, 6_i32, 8_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector2::new(1_i32, 2_i32);
        let c1 = Vector2::new(3_i32, 4_i32);
        let c2 = Vector2::new(5_i32, 6_i32);
        let c3 = Vector2::new(7_i32, 8_i32);
        let columns = [c0, c1, c2, c3];
        let expected = Matrix2x4::new(
            1_i32, 2_i32,
            3_i32, 4_i32,
            5_i32, 6_i32,
            7_i32, 8_i32
        );
        let result = Matrix2x4::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let r1 = Vector4::new(5_i32, 6_i32, 7_i32, 8_i32);
        let rows = [r0, r1];
        let expected = Matrix2x4::new(
            1_i32, 5_i32,
            2_i32, 6_i32,
            3_i32, 7_i32,
            4_i32, 8_i32
        );
        let result = Matrix2x4::from_rows(&rows);

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix4x2_tests {
    use cglinalg_core::{
        Vector4,
        Vector2,
        Matrix2x2,
        Matrix2x4,
        Matrix4x2,
        Matrix4x4,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[2][4], matrix[2][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32, 
            6_i32, 7_i32, 8_i32, 9_i32
        );
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix4x2::new(
            33_i32, 54_i32,  19_i32, 345_i32,
            5_i32,  793_i32, 23_i32, 324_i32
        );
        let zero_matrix2x2 = Matrix2x2::zero();
        let zero_matrix3x2 = Matrix4x2::zero();

        assert_eq!(matrix * zero_matrix2x2, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix4x2::new(
            33_i32, 54_i32,  19_i32, 29_i32,
            5_i32,  234_i32, 98_i32, 7_i32
        );
        let zero = 0_i32;
        let zero_matrix3x2 = Matrix4x2::zero();

        assert_eq!(zero * matrix, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix4x2 = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32,
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let matrix2x2 = Matrix2x2::new(
            1_i32, 2_i32, 
            4_i32, 5_i32
        );
        let expected = Matrix4x2::new(
            12_i32, 15_i32, 18_i32, 21_i32,
            33_i32, 42_i32, 51_i32, 60_i32
        );
        let result = matrix4x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix4x2 = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 8_i32,
            7_i32, 10_i32
        );
        let expected = Matrix4x4::new(
            12_i32, 15_i32, 18_i32, 21_i32,
            26_i32, 33_i32, 40_i32, 47_i32,
            50_i32, 63_i32, 76_i32, 89_i32,
            64_i32, 81_i32, 98_i32, 115_i32
        );
        let result = matrix4x2 * matrix2x4;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix4x2 = Matrix4x2::new(
            4_i32, 5_i32, 6_i32,  7_i32, 
            8_i32, 9_i32, 10_i32, 11_i32
        );
        let vector = Vector2::new(9_i32, -6_i32);
        let expected = Vector4::new(-12_i32, -9_i32, -6_i32, -3_i32);
        let result = matrix4x2 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix4x2 = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            4_i32, 5_i32, 7_i32, 8_i32,
        );
        let scalar = 13_i32;
        let expected = Matrix4x2::new(
            13_i32, 26_i32, 39_i32, 52_i32,
            52_i32, 65_i32, 91_i32, 104_i32
        );
        let result = matrix4x2 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix4x2 = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32,
            4_i32, 5_i32, 7_i32, 8_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x2::new(
            13_i32, 26_i32, 39_i32, 52_i32,
            52_i32, 65_i32, 91_i32, 104_i32
        );
        let result = scalar * matrix4x2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix4x2 = Matrix4x2::zero();
        let matrix = Matrix4x2::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32
        );

        assert_eq!(matrix + zero_matrix4x2, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix4x2 = Matrix4x2::zero();
        let matrix = Matrix4x2::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32
        );

        assert_eq!(zero_matrix4x2 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix4x2::new(
            23_i32, 76_i32,  89_i32, 11_i32,
            34_i32, 324_i32, 75_i32, 62_i32
        );
        let matrix2 = Matrix4x2::new(
            1_i32,  5_i32,  9_i32,  82_i32,
            13_i32, 17_i32, 21_i32, 6_i32
        );
        let expected = Matrix4x2::new(
            24_i32, 81_i32, 98_i32,  93_i32,
            47_i32, 341_i32, 96_i32, 68_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix4x2::new(
            3_i32,  6_i32,  9_i32,  65_i32,
            12_i32, 15_i32, 18_i32, 333_i32
        );
        let matrix2 = Matrix4x2::new(
            1_i32, 15_i32,  29_i32, 27_i32,
            6_i32, 234_i32, 93_i32, 38_i32
        );
        let expected = Matrix4x2::new(
            2_i32, -9_i32,   -20_i32, 38_i32,
            6_i32, -219_i32, -75_i32, 295_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix4x2::new(
            3_i32,  6_i32,  9_i32,  12_i32,
            12_i32, 15_i32, 18_i32, 21_i32
        );
        let zero_matrix3x2 = Matrix4x2::zero();

        assert_eq!(matrix - matrix, zero_matrix3x2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_transpose() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32,
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let expected = Matrix2x4::new(
            1_i32, 5_i32,
            2_i32, 6_i32,
            3_i32, 7_i32,
            4_i32, 8_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let c1 = Vector4::new(5_i32, 6_i32, 7_i32, 8_i32);
        let columns = [c0, c1];
        let expected = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32,
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let result = Matrix4x2::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector2::new(1_i32, 2_i32);
        let r1 = Vector2::new(3_i32, 4_i32);
        let r2 = Vector2::new(5_i32, 6_i32);
        let r3 = Vector2::new(7_i32, 8_i32);
        let rows = [r0, r1, r2, r3];
        let expected = Matrix4x2::new(
            1_i32, 3_i32, 5_i32, 7_i32,
            2_i32, 4_i32, 6_i32, 8_i32
        );
        let result = Matrix4x2::from_rows(&rows);

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix3x4_tests {
    use cglinalg_core::{
        Vector4,
        Vector3,
        Matrix3x4,
        Matrix4x3,
        Matrix4x4,
        Matrix3x3,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
        assert_eq!(matrix[2][0], 7_i32);
        assert_eq!(matrix[2][1], 8_i32);
        assert_eq!(matrix[2][2], 9_i32);
        assert_eq!(matrix[3][0], 10_i32);
        assert_eq!(matrix[3][1], 11_i32);
        assert_eq!(matrix[3][2], 12_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[4][3], matrix[4][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero_matrix4x4 = Matrix4x4::zero();
        let zero_matrix3x4 = Matrix3x4::zero();

        assert_eq!(matrix * zero_matrix4x4, zero_matrix3x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_matrix_times_matrix_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero_matrix3x3: Matrix3x3<i32> = Matrix3x3::zero();
        let zero_matrix3x4: Matrix3x4<i32> = Matrix3x4::zero();

        assert_eq!(zero_matrix3x3 * matrix, zero_matrix3x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero = 0_i32;
        let zero_matrix3x4 = Matrix3x4::zero();

        assert_eq!(zero * matrix, zero_matrix3x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix3x4 = Matrix3x4::new(
            2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,
            8_i32,  9_i32,  10_i32,
            11_i32, 12_i32, 13_i32
        );
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix3x4::new(
            80_i32,  90_i32,  100_i32,
            184_i32, 210_i32, 236_i32,
            288_i32, 330_i32, 372_i32,
            392_i32, 450_i32, 508_i32
        );
        let result = matrix3x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix3x4 = Matrix3x4::new(
            4_i32,  5_i32,  6_i32, 
            7_i32,  8_i32,  9_i32, 
            10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32
        );
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -24_i32);
        let expected = Vector3::new(-354_i32, -375_i32, -396_i32);
        let result = matrix3x4 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix3x4 = Matrix3x4::new(
            2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32, 
            8_i32,  9_i32,  10_i32,
            11_i32, 12_i32, 13_i32
        );
        let matrix4x3 = Matrix4x3::new(
            9_i32, -6_i32,  12_i32, 4_i32,
            35_i32,	96_i32,	27_i32, 4_i32,
            87_i32,	8_i32,  80_i32, 70_i32
        );
        let expected = Matrix3x3::new(
            128_i32,  147_i32,  166_i32,
            810_i32,  972_i32,  1134_i32,
            1624_i32, 1869_i32, 2114_i32
        );
        let result = matrix3x4 * matrix4x3;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x4::new(
            13_i32,  26_i32,  39_i32, 
            52_i32,  65_i32,  78_i32,
            91_i32,  104_i32, 117_i32,
            130_i32, 143_i32, 156_i32 
        );
        let result = matrix3x4 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x4::new(
            13_i32,  26_i32,  39_i32, 
            52_i32,  65_i32,  78_i32,
            91_i32,  104_i32, 117_i32,
            130_i32, 143_i32, 156_i32 
        );
        let result = scalar * matrix3x4;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix3x4 = Matrix3x4::zero();
        let matrix = Matrix3x4::new(
            3684_i32, 42746_i32, 2389_i32,
            345_i32,  546_i32,   234_i32,
            76_i32,   167_i32,   890_i32,
            415_i32,  251_i32,   2340_i32
        );

        assert_eq!(matrix + zero_matrix3x4, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix3x4 = Matrix3x4::zero();
        let matrix = Matrix3x4::new(
            3684_i32, 42746_i32, 2389_i32,
            345_i32,  546_i32,   234_i32,
            76_i32,   167_i32,   890_i32,
            415_i32,  251_i32,   2340_i32
        );

        assert_eq!(zero_matrix3x4 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix3x4::new(
            23_i32,  76_i32,  45_i32,
            89_i32,  34_i32, -21_i32,
            324_i32, 75_i32, -204_i32,
            614_i32, 15_i32,  98_i32
        );
        let matrix2 = Matrix3x4::new(
            1_i32,  5_i32,  23_i32,
            9_i32,  13_i32, 80_i32,
            17_i32, 21_i32, 3_i32,
            87_i32, 41_i32, 34_i32
        );
        let expected = Matrix3x4::new(
            24_i32,  81_i32,  68_i32,
            98_i32,  47_i32,  59_i32,
            341_i32, 96_i32, -201_i32,
            701_i32, 56_i32,  132_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix3x4::new(
            3_i32,  6_i32,  9_i32,
            9_i32,  12_i32, 12_i32,
            15_i32, 18_i32, 15_i32,
            21_i32, 24_i32, 18_i32
        );
        let matrix2 = Matrix3x4::new(
            1_i32,   15_i32, 10_i32,
            29_i32,  6_i32,  71_i32,
            234_i32, 93_i32, 67_i32,
            93_i32,  7_i32,  91_i32
        );
        let expected = Matrix3x4::new(
             2_i32,   -9_i32,  -1_i32,
            -20_i32,   6_i32,  -59_i32,
            -219_i32, -75_i32, -52_i32,
            -72_i32,   17_i32, -73_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix3x4::new(
            3_i32,  6_i32,  9_i32,
            9_i32,  12_i32, 15_i32,
            15_i32, 18_i32, 21_i32,
            21_i32, 24_i32, 27_i32
        );
        let zero_matrix3x4 = Matrix3x4::zero();

        assert_eq!(matrix - matrix, zero_matrix3x4);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix3x4::new(
            1_i32, 2_i32, 3_i32,
            3_i32, 4_i32, 6_i32,
            5_i32, 6_i32, 9_i32,
            7_i32, 8_i32, 12_i32
        );
        let expected = Matrix4x3::new(
            1_i32, 3_i32, 5_i32, 7_i32, 
            2_i32, 4_i32, 6_i32, 8_i32,
            3_i32, 6_i32, 9_i32, 12_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector3::new(1_i32,  2_i32,  3_i32);
        let c1 = Vector3::new(4_i32,  5_i32,  6_i32);
        let c2 = Vector3::new(7_i32,  8_i32,  9_i32);
        let c3 = Vector3::new(10_i32, 11_i32, 12_i32);
        let columns = [c0, c1, c2, c3];
        let expected = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let result = Matrix3x4::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector4::new(1_i32, 2_i32,  3_i32,  4_i32);
        let r1 = Vector4::new(5_i32, 6_i32,  7_i32,  8_i32);
        let r2 = Vector4::new(9_i32, 10_i32, 11_i32, 12_i32);
        let rows = [r0, r1, r2];
        let expected = Matrix3x4::new(
            1_i32, 5_i32, 9_i32,
            2_i32, 6_i32, 10_i32,
            3_i32, 7_i32, 11_i32,
            4_i32, 8_i32, 12_i32
        );
        let result = Matrix3x4::from_rows(&rows);

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix4x3_tests {
    use cglinalg_core::{
        Vector4,
        Vector3,
        Matrix3x3,
        Matrix3x4,
        Matrix4x3,
        Matrix4x4,
    };


    #[rustfmt::skip]
    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
        assert_eq!(matrix[2][0], 9_i32);
        assert_eq!(matrix[2][1], 10_i32);
        assert_eq!(matrix[2][2], 11_i32);
        assert_eq!(matrix[2][3], 12_i32);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds3() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[3][4], matrix[3][4]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds4() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[0][usize::MAX], matrix[0][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds5() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[usize::MAX][0], matrix[usize::MAX][0]);
    }

    #[rustfmt::skip]
    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds6() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[usize::MAX][usize::MAX], matrix[usize::MAX][usize::MAX]);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_identity_equals_matrix() {
        let matrix = Matrix4x3::new(
            2_i32,  3_i32,  4_i32,  5_i32, 
            6_i32,  7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_times_zero_equals_zero() {
        let matrix = Matrix4x3::new(
            33_i32, 54_i32,  19_i32, 345_i32,
            5_i32,  793_i32, 23_i32, 324_i32,
            23_i32, 98_i32,  84_i32, 89_i32
        );
        let zero_matrix3x3 = Matrix3x3::zero();
        let zero_matrix4x3 = Matrix4x3::zero();

        assert_eq!(matrix * zero_matrix3x3, zero_matrix4x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_times_matrix_equals_zero() {
        let matrix = Matrix4x3::new(
            33_i32, 54_i32,  19_i32, 29_i32,
            5_i32,  234_i32, 98_i32, 7_i32,
            23_i32, 98_i32,  84_i32, 89_i32
        );
        let zero = 0_i32;
        let zero_matrix4x3 = Matrix4x3::zero();

        assert_eq!(zero * matrix, zero_matrix4x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication1() {
        let matrix4x3 = Matrix4x3::new(
            2_i32, 3_i32,  4_i32,  5_i32,
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32,
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix4x3::new(
            39_i32,  45_i32,  51_i32,  57_i32,
            87_i32,  102_i32, 117_i32, 132_i32,
            135_i32, 159_i32, 183_i32, 207_i32
        );
        let result = matrix4x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication2() {
        let matrix4x3 = Matrix4x3::new(
            2_i32, 3_i32, 4_i32,  5_i32, 
            5_i32, 6_i32, 7_i32,  8_i32,
            8_i32, 9_i32, 10_i32, 11_i32
        );
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32, 
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32, 
            10_i32, 11_i32, 12_i32
        );
        let expected = Matrix4x4::new(
            36_i32,  42_i32,  48_i32,  54_i32,
            81_i32,  96_i32,  111_i32, 126_i32,
            126_i32, 150_i32, 174_i32, 198_i32,
            171_i32, 204_i32, 237_i32, 270_i32
        );
        let result = matrix4x3 * matrix3x4;
        
        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_multiplication3() {
        let matrix4x3 = Matrix4x3::new(
            4_i32,  5_i32,  6_i32,  7_i32, 
            8_i32,  9_i32,  10_i32, 11_i32,
            12_i32, 13_i32, 14_i32, 15_i32
        );
        let vector = Vector3::new(9_i32, -6_i32, 34_i32);
        let expected = Vector4::new(396_i32, 433_i32, 470_i32, 507_i32);
        let result = matrix4x3 * vector;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix4x3 = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            4_i32, 5_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x3::new(
            13_i32,  26_i32,  39_i32,  52_i32,
            52_i32,  65_i32,  91_i32,  104_i32,
            117_i32, 130_i32, 143_i32, 156_i32
        );
        let result = matrix4x3 * scalar;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix4x3 = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            4_i32, 5_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x3::new(
            13_i32,  26_i32,  39_i32,  52_i32,
            52_i32,  65_i32,  91_i32,  104_i32,
            117_i32, 130_i32, 143_i32, 156_i32
        );
        let result = scalar * matrix4x3;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_matrix4x3 = Matrix4x3::zero();
        let matrix = Matrix4x3::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32,
            320_i32,  2430_i32,  894_i32, 324_i32
        );

        assert_eq!(matrix + zero_matrix4x3, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_matrix4x3 = Matrix4x3::zero();
        let matrix = Matrix4x3::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32,
            320_i32,  2430_i32,  894_i32, 324_i32
        );

        assert_eq!(zero_matrix4x3 + matrix, matrix);
    }

    #[rustfmt::skip]
    #[test]
    fn test_addition() {
        let matrix1 = Matrix4x3::new(
            23_i32, 76_i32,  89_i32, 11_i32,
            34_i32, 324_i32, 75_i32, 62_i32,
            88_i32, 61_i32,  45_i32, 16_i32
        );
        let matrix2 = Matrix4x3::new(
            1_i32,  5_i32,  9_i32,  82_i32,
            13_i32, 17_i32, 21_i32, 6_i32,
            29_i32, 91_i32, 64_i32, 43_i32 
        );
        let expected = Matrix4x3::new(
            24_i32,  81_i32,  98_i32,  93_i32,
            47_i32,  341_i32, 96_i32,  68_i32,
            117_i32, 152_i32, 109_i32, 59_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix4x3::new(
            3_i32,  6_i32,  9_i32,  65_i32,
            12_i32, 15_i32, 18_i32, 333_i32,
            28_i32, 71_i32, 4_i32,  92_i32
        );
        let matrix2 = Matrix4x3::new(
            1_i32, 15_i32,  29_i32, 27_i32,
            6_i32, 234_i32, 93_i32, 38_i32,
            74_i32, 97_i32, 10_i32, 100_i32
        );
        let expected = Matrix4x3::new(
             2_i32,  -9_i32,   -20_i32,  38_i32,
             6_i32,  -219_i32, -75_i32,  295_i32,
            -46_i32, -26_i32,  -6_i32,  -8_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix4x3::new(
            3_i32,  6_i32,  9_i32,  12_i32,
            12_i32, 15_i32, 18_i32, 21_i32,
            34_i32, 17_i32, 8_i32,  84_i32
        );
        let zero_matrix4x3 = Matrix4x3::zero();

        assert_eq!(matrix - matrix, zero_matrix4x3);
    }

    #[rustfmt::skip]
    #[test]
    fn test_transpose() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32,
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let expected = Matrix3x4::new(
            1_i32, 5_i32, 9_i32,
            2_i32, 6_i32, 10_i32,
            3_i32, 7_i32, 11_i32,
            4_i32, 8_i32, 12_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector4::new(1_i32, 2_i32,  3_i32,  4_i32);
        let c1 = Vector4::new(5_i32, 6_i32,  7_i32,  8_i32);
        let c2 = Vector4::new(9_i32, 10_i32, 11_i32, 12_i32);
        let columns = [c0, c1, c2];
        let expected = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32,
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let result = Matrix4x3::from_columns(&columns);

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_construction_from_rows() {
        let r0 = Vector3::new(1_i32,  2_i32,  3_i32);
        let r1 = Vector3::new(4_i32,  5_i32,  6_i32);
        let r2 = Vector3::new(7_i32,  8_i32,  9_i32);
        let r3 = Vector3::new(10_i32, 11_i32, 12_i32);
        let rows = [r0, r1, r2, r3];
        let expected = Matrix4x3::new(
            1_i32, 4_i32, 7_i32, 10_i32,
            2_i32, 5_i32, 8_i32, 11_i32,
            3_i32, 6_i32, 9_i32, 12_i32
        );
        let result = Matrix4x3::from_rows(&rows);

        assert_eq!(result, expected);
    }
}

