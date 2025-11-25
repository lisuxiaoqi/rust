// 定义宏，匹配任意 item
macro_rules! show_item {
    ($item:item) => {
        $item
    };
}

// 使用宏匹配不同类型的 item
show_item! {
    fn hello() {
        println!("Hello from function!");
    }
}

show_item! {
    struct MyStruct {
        x: i32,
        y: i32,
    }
}

show_item! {
    const MY_CONST: i32 = 100;
}

show_item! {
    mod my_mod {
        pub fn mod_func() {
            println!("Hello from module function!");
        }
    }
}

show_item! {
    trait MyTrait {
        fn trait_func(&self);
    }
}

show_item! {
    impl MyTrait for MyStruct {
        fn trait_func(&self) {
            println!("Trait function called on MyStruct");
        }
    }
}

pub fn item_test() {
    hello();

    let s = MyStruct { x: 1, y: 2 };
    println!("MyStruct: x={}, y={}", s.x, s.y);

    println!("MY_CONST: {}", MY_CONST);

    my_mod::mod_func();

    s.trait_func();
}
