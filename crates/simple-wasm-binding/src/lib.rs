#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    unsafe {
        log(a + b);
    }
    a + b
}

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn log(value: i32);
}
