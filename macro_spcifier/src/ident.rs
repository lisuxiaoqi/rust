macro_rules! ident_macro {
    ($name:ident) => {
        fn $name() -> &'static str {
            stringify!($name)
        }
    };
}

ident_macro!(hello);

pub fn ident_test() {
    println!("{}", hello());
}
