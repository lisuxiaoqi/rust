macro_rules! bar {
    ($a:expr) => {
        println!("expr in bar! {}", $a);
    };
}

macro_rules! foo {
    ($a:expr) => {
        forward_macro::bar!($a);
    };
}

pub(crate) use bar;
pub(crate) use foo;
