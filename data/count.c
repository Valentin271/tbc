// Counts up to 1000000000
// This file is intended for {runtime,size,optimizations} comparison with `count.tb`

#define MAX 1000000000

int main() {
    int i = 0;

    while (i < MAX) {
        ++i;
    }

    return 0;
}
