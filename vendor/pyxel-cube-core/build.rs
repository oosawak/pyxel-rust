fn main() {
    println!("cargo::rustc-check-cfg=cfg(pyxel_core)");
    println!("cargo::rustc-cfg=pyxel_core");
}
