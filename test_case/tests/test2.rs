mod common;
use mylib;

#[test]
fn test_mul(){
    common::setup();
    assert_eq!(mylib::mul(3,1),3);
}