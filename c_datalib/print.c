#include "print.h"
#include <stdio.h>
#include <stdlib.h>

void print_data(databox a) {
    if (a.type == NUM) {
        print_double(a.data.num);
    } else if (a.type == STR) {
        print_str(a.data.str);
    } else {
        exit(1);
    }
}

void print_int(int a) {
    printf("%d", a);
}
void print_double(double a) {
    printf("%f", a);
}
void print_str(char* a) {
    printf("%s", a);
}
