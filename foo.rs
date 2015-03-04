#![crate_type = "staticlib"]
#[no_mangle]
pub extern "C" fn foo() {
	println!("hello from rust");
}