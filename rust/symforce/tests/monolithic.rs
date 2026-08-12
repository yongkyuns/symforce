use stack_algebra::Matrix;
use symforce_rust::{FixedSizeLinearization, FixedSizeLinearizationError, Linearization};

#[test]
fn assembles_blocks_in_global_order() {
    let mut monolithic = FixedSizeLinearization::<3, 2, f64>::zeros();

    let first_residual = Matrix::<1, 1, f64>::from_rows([[2.0]]);
    let first_jacobian = Matrix::<1, 1, f64>::from_rows([[3.0]]);
    monolithic
        .add_block(&first_residual, &first_jacobian, &[1])
        .unwrap();

    let second_residual = Matrix::<2, 1, f64>::from_rows([[5.0], [7.0]]);
    let second_jacobian = Matrix::<2, 2, f64>::from_rows([[11.0, 13.0], [17.0, 19.0]]);
    monolithic
        .add_block(&second_residual, &second_jacobian, &[0, 1])
        .unwrap();

    assert_eq!(monolithic.rows(), 3);
    assert_eq!(
        monolithic.residual,
        Matrix::<3, 1, f64>::from_rows([[2.0], [5.0], [7.0]])
    );
    assert_eq!(
        monolithic.jacobian,
        Matrix::<3, 2, f64>::from_rows([[0.0, 3.0], [11.0, 13.0], [17.0, 19.0]])
    );

    let mut linearization = Linearization::<2, f64>::zeros();
    monolithic.commit(&mut linearization).unwrap();
    assert_eq!(linearization.error, 39.0);
    assert_eq!(
        linearization.rhs,
        Matrix::<2, 1, f64>::from_rows([[174.0], [204.0]])
    );
    assert_eq!(
        linearization.hessian,
        Matrix::<2, 2, f64>::from_rows([[410.0, 466.0], [466.0, 539.0]])
    );
}

#[test]
fn rejects_invalid_assembly() {
    let mut monolithic = FixedSizeLinearization::<1, 1, f64>::zeros();
    let residual = Matrix::<1, 1, f64>::from_rows([[1.0]]);
    let jacobian = Matrix::<1, 1, f64>::from_rows([[1.0]]);

    monolithic
        .add_block(&residual, &jacobian, &[1])
        .unwrap_err();
    monolithic.add_block(&residual, &jacobian, &[0]).unwrap();
    assert_eq!(
        monolithic.add_block(&residual, &jacobian, &[0]),
        Err(FixedSizeLinearizationError::ResidualCapacityExceeded {
            row: 1,
            block_rows: 1,
            capacity: 1,
        })
    );

    let mut incomplete = FixedSizeLinearization::<2, 1, f64>::zeros();
    incomplete.add_block(&residual, &jacobian, &[0]).unwrap();
    let mut linearization = Linearization::<1, f64>::zeros();
    assert_eq!(
        incomplete.commit(&mut linearization),
        Err(FixedSizeLinearizationError::Incomplete {
            rows: 1,
            expected: 2,
        })
    );
}
