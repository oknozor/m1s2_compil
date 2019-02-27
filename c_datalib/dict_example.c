#include <stdio.h>
#include "dict.h"

int main() {
    int zero = 0; 
    float one = 1.0; 
    char two[] = "two";
    dictionary *d =dictionary_new();
    dictionary_add(d, "an int", &zero);
    dictionary_add(d, "a float", &one);
    dictionary_add(d, "a string", &two);
    
    printf("record int %i\n", *(int*) dictionary_find(d, "an int"));
    printf("record float %f\n", *(float*) dictionary_find(d, "a float"));
    printf("record int %s\n", (char*) dictionary_find(d, "a string"));
}
