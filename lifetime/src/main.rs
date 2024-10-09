fn main() {
	let r = foo();
	println!("r:{}", r);
}

fn foo()->&'static str{
	let s = "hello".to_string();
	s.as_str()
}
