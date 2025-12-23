#[path = "fallback.rs"]
#[cfg(not(use_cond))]
mod imp;

#[path = "cond.rs"]
#[cfg(use_cond)]
mod imp;

fn main() {
    imp::foo();
}
