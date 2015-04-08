#![crate_type = "staticlib"]
#[no_mangle]
pub extern "C" fn foo() -> u32 {
	println!("hello from rust");
	42
}