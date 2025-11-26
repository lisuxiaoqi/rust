macro_rules! literal_macro {
    ($name:ident, $val: literal) => {
        const $name: i32 = $val;
    };
}

literal_macro!(X, 10);
literal_macro!(Y, 20);
//这样匹配失败，因为是expr，不是literal
//literal_macro!(Z, 20 + 10);

pub fn test_literal_macro() {
    println!("x:{}", X);
    println!("y:{}", Y);
}
