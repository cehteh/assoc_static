use assoc_static::*;

#[test]
fn type_coherency() {
    struct TestType;

    assoc_static!(TestType:TestType, &'static str = "This is the test type");
    assert_eq!(TestType::get_static(), &"This is the test type");

    assoc_static!(TestType:i32, &'static str = "This is i32");
    assert_eq!(i32::get_static(), &"This is i32");
}
