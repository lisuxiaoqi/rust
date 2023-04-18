fn main() {
    // safe computation
    let a : u8 = 255u8;
    let b : u8 = a.wrapping_add(20);
    let c = a.overflowing_add(20);
    // print tuple
    println!("b is {}, c is {:?}", b, c);

    // print format
    println!("a is {}", a);
    println!("\thex format:0x{:x}", a);
    println!("\toct format:0o{:o}", a);
    println!("\tbinary format:0b{:b}", a);
    println!("\tbinary format with padding:0b{:016b}", a);
}
