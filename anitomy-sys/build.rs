extern crate cmake;

fn main() {
    let dst = cmake::build("anitomy-c");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lib/anitomy_c");
}
