mod forward_macro;
mod simple_macro;

fn main() {
    //test_simple_macro();
    test_forward_macro();
}

fn test_simple_macro() {
    simple_macro::foo!(1);
    simple_macro::foo!(1, 3);
    simple_macro::foo!(1, "hello");
}

fn test_forward_macro() {
    forward_macro::foo!(3 + 5);
}
