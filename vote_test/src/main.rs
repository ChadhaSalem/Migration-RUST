#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Fonction de gestion des pannes
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Fonction pour calculer la valeur absolue
fn abs(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

fn vote(
    p_heart_rate_est: &[f32],
    p_confidence_metric: &[f32],
    range_bin_index_shifted: usize,
    thresh: f32
) -> f32 {
    let mut votes = [0.0; 4];
    let mut max_votes = 0.0;
    let mut qualified = 0;

    let start_index = range_bin_index_shifted.saturating_sub(1);
    let end_index = (range_bin_index_shifted + 2).min(p_heart_rate_est.len() - 1);

    for index in start_index..=end_index {
        for index_inner_loop in start_index..=end_index {
            if abs(p_heart_rate_est[index] - p_heart_rate_est[index_inner_loop]) < thresh {
                let vote_index = index.saturating_sub(start_index);
                if vote_index < votes.len() {
                    votes[vote_index] += p_confidence_metric[index_inner_loop];
                }
            }
        }
        let vote_index = index.saturating_sub(start_index);
        if vote_index < votes.len() && votes[vote_index] > max_votes {
            qualified = index;
            max_votes = votes[vote_index];
        }
    }

    p_heart_rate_est.get(qualified).copied().unwrap_or(p_heart_rate_est[0])
}

// Fonction principale
#[no_mangle]
pub extern "C" fn main() -> ! {
    let p_heart_rate_est: [f32; 5] = [1.0, 1.5, 1.2, 1.6, 1.3];
    let p_confidence_metric: [f32; 5] = [0.2, 0.5, 0.3, 0.6, 0.4];
    let range_bin_index_shifted = 2;
    let thresh = 0.5;

    let result = vote(&p_heart_rate_est, &p_confidence_metric, range_bin_index_shifted, thresh);
    
    // Pour afficher le résultat, vous aurez besoin d'un moyen d'envoyer des données à votre environnement d'exécution.
    // Par exemple, vous pourriez utiliser un port série ou un autre moyen de communication.
    
    loop {}
}