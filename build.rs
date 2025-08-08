fn main() {
    // Enable getrandom WASM support for web builds
    if cfg!(target_arch = "wasm32") {
        println!("cargo:rustc-cfg=getrandom_wasm_js");
        println!("cargo:rustc-cfg=web_sys_unstable_apis");
    }
}