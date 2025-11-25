macro_rules! foo{
    ($($a:expr),*)=>{
        $(
            print!("{} ", $a);
        )*
        println!();
    }
}

pub(crate) use foo;
