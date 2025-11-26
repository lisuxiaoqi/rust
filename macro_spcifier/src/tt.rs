macro_rules! tt_macro {
    ($t:tt) => {
        println!("Catched tt:{}", stringify!($t));
    };
}

pub fn test_macro() {
    tt_macro!("hello tt");
    tt_macro!(Somethingstrange);
    tt_macro!([1, 2, 3]);
}
