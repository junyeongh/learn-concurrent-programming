#include <stdlib.h>

int func1() {
    int a = 10;
    int *b = func2(a);
    free(b);
}

int func2(int a) {
    int *temp = (int*)malloc(sizeof(int));
    *temp = a * 20;
    return temp;
}