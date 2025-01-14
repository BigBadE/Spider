mod allocator;
mod test;
mod tools;

#[no_mangle]
pub fn test_print(string: &str) {
    println!("{}", string);
}