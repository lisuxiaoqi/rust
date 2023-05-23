pub fn foo(){
    println!("lib pub foo get called")
}

pub fn foo_indirect(){
    foo_private()
}

fn foo_private(){
    println!("lib private foo get called indirect")
}