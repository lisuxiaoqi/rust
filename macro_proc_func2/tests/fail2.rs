use macro_proc_func2::mul;
fn main() {
    let out = mul!(3; 5);
    eprintln!("out:{}", out);
}
