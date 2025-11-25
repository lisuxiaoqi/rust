macro_rules! life_macro{
    ($name:ident, $life:lifetime)=>{
        fn $name(value: &$life str)->&$life str{
            value
        }
    }
}

life_macro!(echo, 'static);

pub fn test_life_macro() {
    let s: &'static str = "hello world";
    println!("{}", echo(s));
}
