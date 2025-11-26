macro_rules! ty_macro {
    ($name:ident, $t:ty) => {
        type $name = $t;
    };
}

ty_macro!(MyStr, &'static str);
ty_macro!(MyVec, Vec<i32>);

pub fn test_macro() {
    let v: MyVec = vec![1, 2, 3];
    println!("MyVec:{:?}", v);
    let v: MyStr = "hello world";
    println!("MyStr:{}", v)
}
