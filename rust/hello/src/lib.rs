#[no_mangle]
pub extern fn rust_hello_world() -> i32 {
    println!("Hello, I'm in Rust code! I'm about to return 10.");
    10
}
