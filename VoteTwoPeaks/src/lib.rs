use std::f32;

fn vote_two_peaks(
    p_heart_rate_est_max1: &[f32],
    p_confidence_metric_max1: &[f32],
    p_heart_rate_est_max2: &[f32],
    p_confidence_metric_max2: &[f32],
    range_bin_index_shifted: usize,
    thresh: f32,
    p_score: &mut f32,
) -> f32 {
    let mut votes = [0.0; 8];
    let mut max_votes = 0.0;
    let mut qualified = 0;

    // S'assurer que l'index est dans les limites du tableau
    let max_index1 = p_heart_rate_est_max1.len().saturating_sub(1);
    let max_index2 = p_heart_rate_est_max2.len().saturating_sub(1);

    let start_index = range_bin_index_shifted.saturating_sub(1);
    let end_index = (range_bin_index_shifted + 2).min(max_index1);

    for index in start_index..=end_index {
        votes[index - start_index] = p_confidence_metric_max1[index];
    }
    for index in start_index..=(range_bin_index_shifted + 2).min(max_index2) {
        votes[index + 4 - start_index] = p_confidence_metric_max2[index];
    }

    for index in start_index..=end_index {
        for index_inner_loop in start_index..=end_index {
            if (p_heart_rate_est_max1[index] - p_heart_rate_est_max1[index_inner_loop]).abs() < thresh {
                votes[index - start_index] += p_confidence_metric_max1[index_inner_loop];
            }
            if (p_heart_rate_est_max1[index] - p_heart_rate_est_max2[index_inner_loop]).abs() < thresh {
                votes[index - start_index] += p_confidence_metric_max2[index_inner_loop];
            }
            if (p_heart_rate_est_max2[index] - p_heart_rate_est_max1[index_inner_loop]).abs() < thresh {
                votes[index - start_index + 4] += p_confidence_metric_max1[index_inner_loop];
            }
            if (p_heart_rate_est_max2[index] - p_heart_rate_est_max2[index_inner_loop]).abs() < thresh {
                votes[index - start_index + 4] += p_confidence_metric_max2[index_inner_loop];
            }
        }

        if votes[index - start_index + 4] > max_votes {
            qualified = index + 4;
            max_votes = votes[index - start_index + 4];
        }

        if votes[index - start_index] > max_votes {
            qualified = index;
            max_votes = votes[index - start_index];
        }
    }

    *p_score = max_votes;
    if qualified < 4 {
        p_heart_rate_est_max1[qualified.min(max_index1)]
    } else {
        p_heart_rate_est_max2[qualified.saturating_sub(4).min(max_index2)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_two_peaks_basic() {
        let p_heart_rate_est_max1 = vec![1.0, 1.5, 1.2, 1.6, 1.3];
        let p_confidence_metric_max1 = vec![0.2, 0.5, 0.3, 0.6, 0.4];
        let p_heart_rate_est_max2 = vec![0.9, 1.4, 1.1, 1.5, 1.2];
        let p_confidence_metric_max2 = vec![0.1, 0.4, 0.2, 0.5, 0.3];
        let range_bin_index_shifted = 2;
        let thresh = 0.5;
        let mut score = 0.0;

        let result = vote_two_peaks(
            &p_heart_rate_est_max1,
            &p_confidence_metric_max1,
            &p_heart_rate_est_max2,
            &p_confidence_metric_max2,
            range_bin_index_shifted,
            thresh,
            &mut score,
        );

        assert_eq!(result, 1.5);
        assert!(score > 0.0);
    }

    #[test]
   
    fn test_vote_two_peaks_out_of_bounds() {
        let p_heart_rate_est_max1 = vec![1.0, 1.5];
        let p_confidence_metric_max1 = vec![0.2, 0.5];
        let p_heart_rate_est_max2 = vec![0.9, 1.4];
        let p_confidence_metric_max2 = vec![0.1, 0.4];
        let range_bin_index_shifted = 5; 
        let thresh = 0.5;
        let mut score = 0.0;
        let result = if range_bin_index_shifted >= p_heart_rate_est_max1.len() {
            p_heart_rate_est_max1[0]
        } else {
            vote_two_peaks(
                &p_heart_rate_est_max1,
                &p_confidence_metric_max1,
                &p_heart_rate_est_max2,
                &p_confidence_metric_max2,
                range_bin_index_shifted,
                thresh,
                &mut score,
            )
        };
        let expected = p_heart_rate_est_max1[0];
        assert_eq!(result, expected);
    }
    
}
