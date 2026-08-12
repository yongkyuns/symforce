use stack_algebra::Matrix;
use symforce_rust::Pose2;
use symforce_rust::{FactorIndex, Value, Values, ValuesError};

#[test]
fn state_index_preserves_requested_key_order_and_retracts_manifolds() {
    let mut values = Values::new();
    values.insert("pose", Value::Pose2(Pose2::identity()));
    values.insert("bias", Value::Scalar(2.0_f64));
    values.insert(
        "landmark",
        Value::Vector2(Matrix::from_rows([[1.0], [2.0]])),
    );

    let keys = vec!["bias".to_owned(), "pose".to_owned(), "landmark".to_owned()];
    let index = values.state_index(&keys).unwrap();
    assert_eq!(index.tangent_dim(), 6);
    assert_eq!(index.entries()[0].offset, 0);
    assert_eq!(index.entries()[1].offset, 1);
    assert_eq!(index.entries()[2].offset, 4);

    let factor_index =
        FactorIndex::from_state_index(&index, &["pose".to_owned(), "bias".to_owned()]).unwrap();
    assert_eq!(factor_index.columns(), &[1, 2, 3, 0]);

    let updated = values
        .retract(&index, &[1.0, 0.1, 0.2, 0.3, 4.0, 5.0], f64::EPSILON)
        .unwrap();
    assert_eq!(*updated.scalar("bias").unwrap(), 3.0);
    assert_eq!(updated.vector2("landmark").unwrap()[0], 5.0);
    assert_eq!(updated.vector2("landmark").unwrap()[1], 7.0);
}

#[test]
fn typed_access_rejects_wrong_value_kind() {
    let mut values = Values::new();
    values.insert("value", Value::Scalar(1.0_f64));
    assert_eq!(
        values.vector3("value"),
        Err(ValuesError::TypeMismatch("value".to_owned()))
    );
}
