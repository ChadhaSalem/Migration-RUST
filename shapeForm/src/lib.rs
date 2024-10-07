#![no_main]
#![no_std]

use core::panic::PanicInfo;

const MAX_PEAKS: usize = 10; // Adjust size as needed

#[no_mangle]
pub fn shape_form_recognition(
    p_data_in: &[f32],
    p_peak_locs: &mut [u16; MAX_PEAKS],
    p_peak_values: &mut [f32; MAX_PEAKS],
    start_index: u16,
    end_index: u16,
    win_min: u16,
    win_max: u16,
) -> u16 {
    let mut num_peaks: u16 = 0;
    let mut temp_start: Option<u16> = None; 
    let mut in_maxima = false;

    if start_index >= end_index || end_index as usize >= p_data_in.len() {
        return 0;
    }

    for temp in (start_index + 1)..(end_index - 1) {
        let temp = temp as usize;

        if p_data_in[temp] > p_data_in[temp - 1] && p_data_in[temp] > p_data_in[temp + 1] {
            in_maxima = true;
            temp_start = Some(temp as u16);
        } else if p_data_in[temp] < p_data_in[temp - 1] && p_data_in[temp] < p_data_in[temp + 1] {
            if in_maxima {
                if let Some(start) = temp_start {
                    let peak_width = (temp as u16) - start;
                    if peak_width >= win_min && peak_width <= win_max {
                        if (num_peaks as usize) < MAX_PEAKS {
                            p_peak_locs[num_peaks as usize] = start;
                            p_peak_values[num_peaks as usize] = p_data_in[start as usize];
                            num_peaks += 1;
                        }
                    }
                }
                in_maxima = false;
                temp_start = None;
            }
        }
    }
    num_peaks
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_form_recognition_no_peaks() {
        let p_data_in: [f32; 10] = [0.0; 10];
        let mut p_peak_locs: [u16; MAX_PEAKS] = [0; MAX_PEAKS];
        let mut p_peak_values: [f32; MAX_PEAKS] = [0.0; MAX_PEAKS];
        let start_index: u16 = 1;
        let end_index: u16 = 8;
        let win_min: u16 = 1;
        let win_max: u16 = 5;

        let num_peaks = shape_form_recognition(
            &p_data_in,
            &mut p_peak_locs,
            &mut p_peak_values,
            start_index,
            end_index,
            win_min,
            win_max,
        );
        assert_eq!(num_peaks, 0);
        assert_eq!(p_peak_locs, [0; MAX_PEAKS]);
        assert_eq!(p_peak_values, [0.0; MAX_PEAKS]);
    }

    #[test]
    fn test_shape_form_recognition_one_peak() {
        let p_data_in: [f32; 10] = [0.0, 0.1, 0.2, 0.1, 0.0, -0.1, -0.2, -0.1, 0.0, 0.1];
        let mut p_peak_locs: [u16; MAX_PEAKS] = [0; MAX_PEAKS];
        let mut p_peak_values: [f32; MAX_PEAKS] = [0.0; MAX_PEAKS];
        let start_index: u16 = 1;
        let end_index: u16 = 8;
        let win_min: u16 = 1;
        let win_max: u16 = 5;

        let num_peaks = shape_form_recognition(
            &p_data_in,
            &mut p_peak_locs,
            &mut p_peak_values,
            start_index,
            end_index,
            win_min,
            win_max,
        );

        assert_eq!(num_peaks, 1);
        assert_eq!(p_peak_locs[0], 2);
        assert_eq!(p_peak_values[0], 0.2);
    }

    #[test]
    fn test_shape_form_recognition_invalid_indices() {
        let p_data_in: [f32; 10] = [0.0, 0.1, 0.2, 0.1, 0.0, -0.1, -0.2, -0.1, 0.0, 0.1];
        let mut p_peak_locs: [u16; MAX_PEAKS] = [0; MAX_PEAKS];
        let mut p_peak_values: [f32; MAX_PEAKS] = [0.0; MAX_PEAKS];
        let start_index: u16 = 8;
        let end_index: u16 = 1;
        let win_min: u16 = 1;
        let win_max: u16 = 5;

        let num_peaks = shape_form_recognition(
            &p_data_in,
            &mut p_peak_locs,
            &mut p_peak_values,
            start_index,
            end_index,
            win_min,
            win_max,
        );

        assert_eq!(num_peaks, 0);
        assert_eq!(p_peak_locs, [0; MAX_PEAKS]);
        assert_eq!(p_peak_values, [0.0; MAX_PEAKS]);
    }
}
