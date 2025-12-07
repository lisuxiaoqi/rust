use macro_proc_func::*;

fn main() {
    let _ = add!(1 + 3);
    let _ = add!(1);
    let _ = add!(1 + 3, 4, 5);
}
