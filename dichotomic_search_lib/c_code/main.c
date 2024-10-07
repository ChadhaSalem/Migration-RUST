#include <stdio.h>

// Déclarer le fct Rust 
extern long dichotomic_search(const int* array, size_t len, int target);     

int main() {
  int array[] = {0, 1, 2, 3, 4, 5, 6};
    size_t len = sizeof(array) / sizeof(array[0]);
    int element = 3;

    // Appeler la fct Rust
    long index = dichotomic_search(array, len, element);
    if (index >= 0) {
        printf("Element found in position  %ld\n", index);
    } else {
        printf("Element not found\n");
    }

    return 0;
}



