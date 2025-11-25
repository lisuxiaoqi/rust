macro_rules! time_info {
    ($($s:stmt);*) => {
        $(
           println!("Before stmt");
           println!("Catched stmt:{}", stringify!($s));
           $s
           println!("After stmt");
        )*
    };
}

pub fn stmt_test() {
    time_info! {
        let s = "hello world";
        if true{
            println!("stmt 2:{}", s)
        }
    }
}
