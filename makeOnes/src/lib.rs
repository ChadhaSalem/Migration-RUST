#![no_std]
#![no_main]

use core::cmp::{max, min};

/// Calcule le plafond d'un nombre flottant en tant qu'entier.
fn ceil(x: f32) -> i16 {
    let int_part = x as i16;
    let decimal_part = x - int_part as f32;
    if decimal_part > 0.0 {
        int_part + 1
    } else {
        int_part
    }
}

#[no_mangle]
pub extern "C" fn make_ones(p_data: *mut u8, start_ind: i16, end_ind: i16, sample_freq_hz: f32) {
    unsafe {
        let max_end_ind = ceil(4.0 / sample_freq_hz * 1024.0);

        let start_ind = max(start_ind, 0);
        let end_ind = min(end_ind, max_end_ind) as usize;

        // Convertir le pointeur brut en une slice mutable
        let p_data_len = {
            let len = p_data as usize;
            let len = len as usize; // Suppose la taille maximale de la mémoire est 0x10000 (64 Ko)
            len
        };

        let end_ind = min(end_ind, p_data_len - 1);
        let start_ind = start_ind as usize;

        if start_ind >= p_data_len {
            return;
        }

        // Manipuler le pointeur comme une slice mutable
        let data = core::slice::from_raw_parts_mut(p_data, p_data_len);

        for index_temp in start_ind..=end_ind {
            data[index_temp] = 1;
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_ones_basic() {
        let mut data = vec![0; 10];
        unsafe {
            make_ones(data.as_mut_ptr(), 2, 5, 1.0);
        }
        assert_eq!(data[2], 1);
        assert_eq!(data[3], 1);
        assert_eq!(data[4], 1);
        assert_eq!(data[5], 1);
        assert_eq!(data[6], 0);
    }

    #[test]
    fn test_make_ones_start_ind_out_of_bounds() {
        let mut data = vec![0; 10];
        unsafe {
            make_ones(data.as_mut_ptr(), 15, 18, 1.0);
        }
        assert_eq!(data, vec![0; 10]);
    }

    #[test]
    fn test_make_ones_end_ind_out_of_bounds() {
        let mut data = vec![0; 10];
        unsafe {
            make_ones(data.as_mut_ptr(), 2, 15, 1.0);
        }
        assert_eq!(data[2], 1);
        assert_eq!(data[3], 1);
        assert_eq!(data[4], 1);
        assert_eq!(data[5], 1);
        assert_eq!(data[6], 1);
        assert_eq!(data[7], 1);
        assert_eq!(data[8], 1);
        assert_eq!(data[9], 1);
    }

    #[test]
    fn test_make_ones_adjusted_indices() {
        let mut data = vec![0; 10];
        unsafe {
            make_ones(data.as_mut_ptr(), -1, 12, 2.0);
        }
        for i in 0..10 {
            assert_eq!(data[i], 1);
        }
    }
}
