#include "databox.h"
#include <assert.h>
#include <stdlib.h>
#include <assert.h>

databox new_object() {
    databox d = {.data.dict = dictionary_new(), .type= DICT};
    return d;
}

databox new_from_int(int a) {
    return new_from_double((double) a);
}

databox new_from_double(double a) {
    databox d = {.data.num = a, .type = NUM};
    return d;
}

databox new_from_str(char *a) {
    databox d = {.data.str = a, .type = STR};
    return d;
}

databox copy(databox a) {
    return a;
}

void increment(databox *a) {
    if (a->type == NUM) {
        (a->data).num++;
    }
}

int data_eq_data(databox a, databox b) {
    if (a.type == NUM && b.type == NUM) {
        return a.data.num == b.data.num;
    } else if (a.type == STR && b.type == STR) {
        unsigned int a_len = strlen(a.data.str);
        unsigned int b_len = strlen(b.data.str);
        return a_len == b_len;
    } else {
        exit(1);
    }
}

// String parse in C ?
int double_eq_data(double a, databox b) {
    if (b.type == NUM) {
        return a == b.data.num;
    } else {
        exit(1);
    }
}

int data_eq_double(databox a, double b) {
    if (a.type == NUM) {
        return a.data.num == b;
    } else {
        exit(1);
    }
}

int int_eq_data(int a, databox b) {
    return double_eq_data((double) a, b);
}

int data_eq_int(databox a, int b) {
    return data_eq_double(a, (double) b);
}

int data_neq_data(databox a, databox b) {
    return reverse(data_eq_data(a, b));
}

int double_neq_data(double a, databox b) {
    return reverse(double_eq_data(a, b));
}

int data_neq_double(databox a, double b) {
    return reverse(data_eq_double(a, b));
}

int int_neq_data(int a, databox b) {
    return reverse(int_eq_data(a, b));
}

int data_neq_int(databox a, int b) {
    return reverse(data_eq_int(a, b));
}

int reverse(int boolean) {
    if (boolean == 1) return 0;
    else return 1;
}

// Add
databox data_add_data(databox a, databox b) {
    if (a.type == NUM && b.type == NUM) {
        double result = a.data.num + b.data.num;
        databox out = {.data.num = result};
        return out;

    } else if (a.type == STR && b.type == STR) {
        char temp[sizeof(a.data.str) + sizeof(b.data.str)];
        strcpy(temp, a.data.str);
        strcat(temp, b.data.str);
        databox out = {.data.str = temp};
        return out;
    } else {
        exit(1);
    }
}

databox double_add_data(double a, databox b) {
    databox a_as_data = {.data.num = a};
    databox out = data_add_data(b, a_as_data);
    return out;
}

databox data_add_double(databox a, double b) {
    return double_add_data(b, a);
}

databox int_add_data(int a, databox b) {
    return double_add_data((double) a, b);
}

databox data_add_int(databox a, int b) {
    return data_add_double(a, (double) b);
}

// Sub
databox data_sub_data(databox a, databox b) {
    if (a.type == NUM && b.type == NUM) {
        double result = a.data.num - b.data.num;
        databox out = {.data.num = result};
        return out;
    } else {
        exit(1);
    }
}

databox double_sub_data(double a, databox b) {
    databox a_as_data = {.data.num = a};
    databox out = data_sub_data(b, a_as_data);
    return out;
}

databox data_sub_double(databox a, double b) {
    return double_sub_data(b, a);
}

databox int_sub_data(int a, databox b) {
    return double_sub_data((double) a, b);
}

databox data_sub_int(databox a, int b) {
    return data_sub_double(a, (double) b);
}

databox data_mul_data(const databox a, const databox b) {
    assert(a.type == NUM && b.type == NUM);
    double result = a.data.num * b.data.num;
    databox out = {.data.num = result, .type=NUM};
    return out;
}

databox double_mul_data(const double a, const databox b) {
    assert(b.type == NUM);
    databox a_as_data = {.data.num = a, .type=NUM};
    databox out = data_mul_data(b, a_as_data);
    return out;
}

databox data_mul_double(const databox a, const double b) {
    return double_mul_data(b, a);
}

databox int_mul_data(const int a, const databox b) {
    return double_mul_data((double) a, b);
}

databox data_mul_int(const databox a, const int b) {
    return double_mul_data((double) b, a);
}

int data_greater_than_data(databox a, databox b) {
    if (a.type == NUM && b.type == NUM) {
        return a.data.num > b.data.num;
    } else if (a.type == STR && b.type == STR) {
        unsigned int a_len = strlen(a.data.str);
        unsigned int b_len = strlen(b.data.str);
        return a_len > b_len;
    } else {
        exit(1);
    }
}

int double_greater_than_data(double a, databox b) {
    if (b.type == NUM) {
        return a > b.data.num;

    } else {
        return a > strlen(b.data.str);
    }
}

int data_greater_than_double(databox a, double b) {
    if (a.type == NUM) {
        return a.data.num > b;

    } else {
        return strlen(a.data.str) > b;
    }
}

int int_greater_than_data(int a, databox b) {
    double a_as_double = (double) a;
    return double_greater_than_data(a_as_double, b);
}

int data_greater_than_int(databox a, int b) {
    double b_as_double = (double) b;
    return data_greater_than_double(a, b_as_double);
}

int data_less_than_data(databox a, databox b) {
    return data_greater_than_data(b, a);
}

int double_less_than_data(double a, databox b) {
    return data_greater_than_double(b, a);
}

int data_less_than_double(databox a, double b) {
    return double_greater_than_data(b, a);
}

int int_less_than_data(int a, databox b) {
    return data_greater_than_int(b, a);
}

int data_less_than_int(databox a, int b) {
    return int_greater_than_data(b, a);
}


