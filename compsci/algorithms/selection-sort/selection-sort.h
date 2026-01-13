#ifndef SELECTION_SORT_H
#define SELECTION_SORT_H

#define SWAP(x, y) \
    {              \
        int t;     \
        t = x;     \
        x = y;     \
        y = t;     \
    }

void selection_sort(int *arr, int n) {
    for (int i = 0; i < n - 1; i++) {
        int least = i;

        for (int j = i + 1; j < n; j++)
            if (arr[j] < arr[least]) least = j;
        SWAP(arr[i], arr[least]);
    }
}

#endif
