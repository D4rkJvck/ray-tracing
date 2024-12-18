use rt::Vector;

#[test]
fn test_vectors_add() {
    let v1 = Vector::new(2.0, 1.0, 3.0);
    let v2 = Vector::new(1.0, 2.0, 0.0);

    assert_eq!(v1 + v2, Vector::new(3.0, 3.0, 3.0));
}

#[test]
fn test_vectors_sub() {
    let v1 = Vector::new(1.0, 3.0, 2.0);
    let v2 = Vector::new(1.0, 3.0, 2.0);

    assert_eq!(v1 - v2, Vector::default())
}