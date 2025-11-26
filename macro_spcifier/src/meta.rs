macro_rules! meta_macro {
    (#[$m:meta] $i:item) => {
        #[$m]
        $i
    };
}

meta_macro! {
    #[cfg(all(unix))]
    fn foo() {
        println!("hello meta");
    }
}

pub fn test_meta() {
    foo();
}
