macro_rules! pat_macro {
    ($val:expr, $p:pat) => {
        match $val {
            $p => println!("Matched pattern:{}", stringify!($p)),
            _ => println!("No Match"),
        }
    };
}

pub fn test_pat() {
    let v = Some(10);
    pat_macro!(v, Some(_a));
    pat_macro!(v, None);
    pat_macro!(10, 0 | 5 | 10);
}
