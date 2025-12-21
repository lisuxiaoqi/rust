use macro_attr::log_call;

#[log_call(subattr1 = "what happened", subattr2, subattr3(milk, tea))]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let _r = add(3, 5);
}
