use rt::Vector;

#[test]
fn test_add_2_vectors() {
    let v1 = Vector::new(2.0, 1.0, 3.0);
    let v2 = Vector::new(1.0, 2.0, 0.0);

    assert_eq!(v1 + v2, Vector::new(3.0, 3.0, 3.0));
}

#[test]
fn test_add_3_vectors() {
    let v1 = Vector::new(2.0, 1.0, 3.0);
    let v2 = Vector::new(1.0, 2.0, 0.0);
    let v3 = Vector::new(3.0, 3.0, 3.0);

    assert_eq!(v1 + v2 + v3, Vector::new(6.0, 6.0, 6.0))
}

#[test]
fn test_sub_vectors() {
    let v1 = Vector::new(1.0, 3.0, 2.0);
    let v2 = Vector::new(1.0, 3.0, 2.0);

    assert_eq!(v1 - v2, Vector::default())
}

#[test]
fn test_mul_vectors(){
    let v1= Vector::new(2.0, 4.0, 2.0);
    let v2 = Vector::new(3.0, 2.0, 4.0);
    assert_eq!(v1 * v2, Vector::new(6.0,8.0,8.0))
}

#[test]
fn test_mul_vector_and_scalar(){
    let v1 = Vector::new(2.0, 4.0, 2.0);
    assert_eq!(v1 * 3.0, Vector::new(6.0, 12.0, 6.0))
   

}


#[test]
fn test_div_vector_and_scalar() {
    let v1 = Vector::new(6.0, 12.0, 6.0);
    assert_eq!(v1 / 3.0, Vector::new(2.0, 4.0, 2.0))
}

#[test]
fn test_neg_vector() {
    let v1 = Vector::new(2.0, -4.0, 3.0);
    assert_eq!(-v1, Vector::new(-2.0, 4.0, -3.0))
}

#[test]
fn test_add_assign() {
    let mut v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(4.0, 5.0, 6.0);
    v1 += v2;
    assert_eq!(v1, Vector::new(5.0, 7.0, 9.0));
}

#[test]
fn test_mul_assign() {
    let mut v1 = Vector::new(1.0, 2.0, 3.0);
    v1 *= 2.0;
    assert_eq!(v1, Vector::new(2.0, 4.0, 6.0));
}

#[test]
fn test_div_assign() {
    let mut v1 = Vector::new(2.0, 4.0, 6.0);
    v1 /= 2.0;
    assert_eq!(v1, Vector::new(1.0, 2.0, 3.0));
}