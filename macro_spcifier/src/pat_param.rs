macro_rules! pat_param_macro {
    ($val:expr, $p:pat_param) => {
        match $val {
            $p => println!("Matched pattern:{}", stringify!($p)),
            _ => println!("No Match"),
        }
    };
}

pub fn test_macro() {
    let v = Some(10);
    pat_param_macro!(v, Some(_a));
    pat_param_macro!(v, None);
    //or pattern fails here
    //pat_param_macro!(10, 0 | 5 | 10);
}
