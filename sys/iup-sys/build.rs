#![feature(static_nobundle)]

fn main() {
    if let Ok(path) = std::env::var("RUST_IUP_STATIC_LIBS") {
        println!("cargo:rustc-flags=-L {}", path);
    }
    println!("cargo:rustc-flags=-l static=iup");
    println!("cargo:rustc-flags=-l static-nobundle=ole32");
    println!("cargo:rustc-flags=-l static-nobundle=comctl32");
    println!("cargo:rustc-flags=-l static-nobundle=shell32");
    println!("cargo:rustc-flags=-l static-nobundle=comdlg32");
}
