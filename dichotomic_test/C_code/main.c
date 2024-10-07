#include <stdio.h>

extern long dichotomic_test(const int* array, size_t len, int target);

int main() {
    int arr[] = {1, 3, 5, 7, 9};
    size_t len = sizeof(arr) / sizeof(arr[0]);
    int target = 5;

    // Appeler la fonction Rust
    long index = dichotomic_test(arr, len, target);

    // Afficher le résultat
    if (index >= 0) {
        printf("Element found at position %ld\n", index);
    } else {
        printf("Element not found\n");
    }

    printf("Hello, World!\n");
    return 0;
}
