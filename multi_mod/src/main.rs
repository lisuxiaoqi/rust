mod mod_same_level;
use mod_same_level as msl;
use mod_same_level::foo;
use mod_same_level::foo as poo;

mod mod_folder;
mod any_folder;

fn main() {
    // use mod at the same level
    mod_same_level::foo();
    msl::foo();
    foo();
    poo();

    // use mod at sub folder
    mod_folder::inner_foo();

    // use mod at any folder
    mod_folder::sub_mod_folder_a::foo();
}
