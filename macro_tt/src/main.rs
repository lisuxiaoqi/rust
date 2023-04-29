macro_rules! foo {
    ($($tt:tt)*) => {
       // println!("{:?}", stringify!($($tt)*));
       $(
            println!("{}", stringify!($tt));
            println!("--");
           )*
    };
}

fn main() {
    foo!(x + y; "string");
    foo!(fn foo() {});
    foo!(struct Foo { x: i32 , y:String, z:{z1:11, z2:"bb"}});
    //注意：大括号中所有内容统一算作一个tt
    foo!(fn Foo (){let i = 3; let b = i+4; println!("hello")});
    //整个最外层大括号算作一个tt
    foo!({
		tx_hash:"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfa",
		fee:10000,
		action:{
			action:"register",
			params:"0x00"
		},
		inputs:[
			{
				index:0,
				capacity:10000
			},
			{
				index:1,
				capacity:10000
			}
		],
		outputs:[
			{
				index:0,
				capacity:10000
			},
			{
				index:1,
				capacity:10000
			},
		],
		digest:"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae"
	});
}

