#include <stdio.h>
#include "selection-sort.h"

int main() {
    int arr[] = {3, 5, 8, 1, 2, 7};
    int n = sizeof(arr) / sizeof(int);

    selection_sort(arr, n);

    for (int i = 0; i < n; i++)
        printf("%d ", arr[i]);
}
