#[no_mangle]
pub extern "C" fn dichotomic_search(array: *const i32, len: usize, element: i32) -> isize {
    let slice = unsafe { std::slice::from_raw_parts(array, len) };   // ici on a utilisé la fct from_raw_parts de std::slice module pour créer le slice
    match slice.binary_search(&element) {
        Ok(index) => index as isize,
        Err(_) => -1,
    }
}
