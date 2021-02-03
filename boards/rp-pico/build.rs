use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let link_x = include_bytes!("link.x");
    let mut script = File::create(out_dir.join("link.x")).unwrap();
    script.write_all(link_x).unwrap();
}
