macro_rules! vis_macro{
    ($v:vis, $name: ident)=>{
        $v fn $name(){
            println!("Called from vis specifier:{}", stringify!($v));
        }
    }
}

vis_macro!(pub, test_macro);
