macro_rules! calc{
    (eval $expr: expr)=>{
        println!("{} = {}", stringify!($expr), $expr);
    };
    (eval $expr: expr, $(eval $remains:expr),+)=>{
        calc!(eval $expr);
        calc!($(eval $remains),+);
    };
}

#[test]
fn test_dsl() {
    calc! {
        eval 1+2,
        eval 2*3,
        eval 3+2*5
    }
}
