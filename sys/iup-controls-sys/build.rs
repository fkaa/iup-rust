#![feature(static_nobundle)]

fn main() {
    if let Ok(path) = std::env::var("RUST_IUP_STATIC_LIBS") {
        println!("cargo:rustc-flags=-L {}", path);
    }
    println!("cargo:rustc-flags=-l static=iupcontrols");
}
