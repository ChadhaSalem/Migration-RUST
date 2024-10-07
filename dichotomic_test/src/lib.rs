
#![no_std]
#![no_main]
use core::slice;


#[no_mangle]
pub extern "C" fn dichotomic_test(array: *const i32, len: usize, element: i32) -> isize {
    let slice = unsafe { slice::from_raw_parts(array, len) };
    match slice.binary_search(&element) {
        Ok(index) => index as isize,
        Err(_) => -1,
    }
}
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_dichotomic_test() {
        let array = [1, 2, 3, 4, 5];
        let len = array.len();
        let element = 3;
        let result = dichotomic_test(array.as_ptr(), len, element);
        assert_eq!(result, 2); // Verifier que l indice de 3 est 2 

        let element_not_found = 6;
        let result_not_found = dichotomic_test(array.as_ptr(), len, element_not_found);
        assert_eq!(result_not_found, -1); // l element 6 not found 
    }
}
