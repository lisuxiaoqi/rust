use macro_derive_list::FieldHelper;

#[derive(FieldHelper, Debug)]
struct TStruct {
    #[add(getter, setter)]
    name: String,
    #[add(setter)]
    age: u32,
    #[add(getter)]
    gender: bool,
}

fn main() {
    let mut t = TStruct {
        name: "zhang".into(),
        age: 20,
        gender: true,
    };

    println!("{:?}", t);

    t.set_name("wang".into());
    t.set_age(40);
    println!("{:?}", t);

    println!("gender:{}", t.gender());
}
