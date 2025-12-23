fn main() {
    let use_cond = true;
    if use_cond {
        println!("cargo:rustc-cfg=use_cond");
    }
    println!("cargo:rustc-check-cfg=cfg(use_cond)");
}
