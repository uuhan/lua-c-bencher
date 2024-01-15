fn main() {
    cc::Build::new().file("c/export.c").compile("export");

    println!("cargo:rerun-if-changed=c/export.c");
}
