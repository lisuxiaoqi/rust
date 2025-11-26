macro_rules! path_macro {
    ($p:path, $value:literal) => {
        $p($value)
    };
}

pub fn test_macro() {
    let v = path_macro!(std::string::String::from, "hello path");
    println!("{}", v);
}
