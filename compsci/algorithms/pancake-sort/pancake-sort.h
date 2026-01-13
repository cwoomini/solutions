#ifndef PANCAKE_SORT_H
#define PANCAKE_SORT_H

#define SWAP(x, y) \
    {              \
        int t;     \
        t = x;     \
        x = y;     \
        y = t;     \
    }

void flip(int *arr, int idx) {
    for (int i = 0, j = idx - 1; i < j; i++, j--)
        SWAP(arr[i], arr[j]);
}

void pancake_sort(int *arr, int n) {
    int max = 0;
    for (int i = 0; i < n; i++)
        if (arr[i] > arr[max])
            max = i;

    flip(arr, max + 1);
    flip(arr, n);

    if (n <= 1) return;
    else pancake_sort(arr, n - 1);
}

#endif
