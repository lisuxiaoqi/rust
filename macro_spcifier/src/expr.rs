macro_rules! expr_macro {
    ($exp:expr) => {
        println!("catched:{}", stringify!($exp))
    };
}

macro_rules! expr_2021_macro {
    ($exp:expr_2021) => {
        println!("catched:{}", stringify!($exp))
    };
}

pub fn expr_test() {
    expr_macro!(1 + 2);
    expr_macro!(_);
    expr_macro!(const { 3 + 4 });
}

pub fn expr_2021_test() {
    expr_2021_macro!(1 + 2);
    //expr_2021不匹配_和const
    //expr_2021_macro!(_);
    //expr_2021_macro!(const { 3 + 4 });
}
