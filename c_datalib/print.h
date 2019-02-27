#include "databox.h"

void print_data(databox a);
void print_int(int a);
void print_double(double a);
void print_str(char* a);
void eprint();

#define print(a) _Generic((a),  \
        databox:  print_data,   \
        int: print_int,         \
        double: print_double,   \
        char*: print_str,       \
        default: eprint) (a)
