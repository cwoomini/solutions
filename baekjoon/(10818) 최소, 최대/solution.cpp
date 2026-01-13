#include <iostream>

#define MAX_VALUE 1000000
#define MIN_VALUE -1000000

int main() {
    int n;
    std::cin >> n;

    int min = MAX_VALUE;
    int max = MIN_VALUE;
    for (int i = 0; i < n; i++) {
        int t;
        std::cin >> t;

        if (t < min) min = t;
        if (t > max) max = t;
    }

    std::cout << min << " " << max << std::endl;
    return 0;
}
