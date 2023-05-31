mod common;

use mylib;

#[test]
fn test_add(){
    common::setup();
    assert_eq!(mylib::sub(3,1),2);
}