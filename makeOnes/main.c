#include <stdint.h>
#include <stdio.h>

extern void make_ones(uint8_t *p_data, int16_t start_ind, int16_t end_ind, float sample_freq_hz);

int main() {
    uint8_t data[10] = {0};
    make_ones(data, 2, 5, 1.0);

    // Afficher les résultats pour vérification
    for (int i = 0; i < 10; i++) {
        printf("%d ", data[i]);
    }
    printf("\n");

    return 0;
}
