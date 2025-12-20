use macro_derive::Builder;

#[derive(Builder, Debug)]
struct TStruct {
    name: String,
    #[default = "fake@gmail.com"]
    email: String,
    #[skip]
    flag: bool,
}

fn main() {
    let b = TStruct::builder().name("Chang".into());
    println!("Builder:{:?}", b);

    let t = b.build();
    println!("Generated struct:{:?}", t);
}
