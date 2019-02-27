#ifndef DATABOX_H
#define DATABOX_H

#include <string.h>

typedef enum type_e {
    NUM,
    STR
} type_e;

typedef union data_u {
    double num;
    char *str;
} data_u;

typedef struct databox {
    data_u data;
    type_e type;
} databox;

databox copy(databox a);

databox new_from_int(int a);

databox new_from_double(double a);

databox new_from_str(char *a);

void decrement(databox *a);

void increment(databox *a);

databox data_add_data(databox a, databox b);

databox double_add_data(double a, databox b);

databox data_add_double(databox a, double b);

databox int_add_data(int a, databox b);

databox data_add_int(databox a, int b);

databox data_sub_data(databox a, databox b);

databox double_sub_data(double a, databox b);

databox data_sub_double(databox a, double b);

databox int_sub_data(int a, databox b);

databox data_sub_int(databox a, int b);

int data_greater_than_data(databox a, databox b);

int double_greater_than_data(double a, databox b);

int data_greater_than_double(databox a, double b);

int double_greater_than_double(double a, double b);

int int_greater_than_data(int a, databox b);

int data_greater_than_int(databox a, int b);

int data_less_than_data(databox a, databox b);

int double_less_than_data(double a, databox b);

int data_less_than_double(databox a, double b);

int double_less_than_double(double a, double b);

int int_less_than_data(int a, databox b);

int data_less_than_int(databox a, int b);

int data_eq_data(databox a, databox b);

int double_eq_data(double a, databox b);

int data_eq_double(databox a, double b);

int double_eq_double(double a, double b);

int int_eq_data(int a, databox b);

int data_eq_int(databox a, int b);

databox double_mul_data(const double a, const databox b);

databox data_mul_double(const databox a, const double b);

databox int_mul_data(const int a, const databox b);

databox data_mul_int(const databox a, const int b);

databox data_mul_data(databox a, databox b);

int def(double a, double b);

int def_2(double a);

// The "ln" comment is just a flag to indicate multiline #define
// otherwise it would be impossible to copy this in the final rust binary

#define new(a) _Generic((a),                            \
        databox: copy,                                  \
        double: new_from_double,                        \
        int: new_from_int,                              \
        char*: new_from_str,                            \
        default: def_2)(a)


#define mul(a, b) _Generic((a),                         \
        double:  mul_given_double(b),                   \
        int: mul_given_int(b),                          \
        databox: mul_given_data(b),                     \
        default: def)((a),(b))

#define mul_given_double(b) _Generic((b),               \
        databox: double_mul_data,                       \
        default: def)

#define mul_given_int(b) _Generic((b),                  \
        databox: data_mul_data,                         \
        default: def)

#define mul_given_data(b) _Generic((b),                 \
        databox: data_mul_data,                         \
        double:  double_greater_than_double,            \
        default: def)


#define add(a, b) _Generic((a),                         \
        double:  add_given_double(b),                   \
        int:     add_given_int(b),                      \
        databox: add_given_data(b),                     \
        default: def)((a),(b))

#define add_given_double(b) _Generic((b),               \
        databox: double_add_data,                       \
        default: def)

#define add_given_data(b) _Generic((b),                 \
        databox: data_add_data,                         \
        double:  data_add_double,                       \
        int:     data_add_int)

#define add_given_int(b) _Generic((b),                  \
        databox: int_add_data,                          \
        default: def)                                   \


#define sub(a, b) _Generic((a),                         \
        double:  sub_given_double(b),                   \
        int:     sub_given_int(b),                      \
        databox: sub_given_data(b),                     \
        default: def)((a),(b))

#define sub_given_double(b) _Generic((b),               \
        databox: double_sub_data,                       \
        default: def)

#define sub_given_data(b) _Generic((b),                 \
        databox: data_sub_data,                         \
        double:  data_sub_double,                       \
        int:     data_sub_int)

#define sub_given_int(b) _Generic((b),                  \
        databox: int_sub_data,                          \
        default: def)                                   \


#define gt(a, b) _Generic((a),                          \
        double:  greater_than_given_double(b),          \
        int: greater_than_given_int(b),                 \
        databox: greater_than_given_data(b),            \
        default: def)((a),(b))

#define greater_than_given_double(b) _Generic((b),      \
        databox: double_greater_than_data,              \
        double:  double_greater_than_double,            \
        default: def)

#define greater_than_given_data(b) _Generic((b),        \
        databox: data_greater_than_data,                \
        double:  data_greater_than_double,              \
        int:  data_greater_than_int)

#define greater_than_given_int(b) _Generic((b),         \
        databox: int_greater_than_data,                 \
        default: def)                                   \


#define lt(a, b) _Generic((a),                          \
        double:  less_than_given_double(b),             \
        int:     less_than_given_int(b),                \
        databox: less_than_given_data(b),               \
        default: def)((a),(b))

#define less_than_given_double(b) _Generic((b),         \
        databox: double_less_than_data,                 \
        double: double_less_than_double,                \
        default: def)

#define less_than_given_data(b) _Generic((b),           \
        databox: data_less_than_data,                   \
        int:     data_less_than_int,                    \
        double:  data_less_than_double)

#define less_than_given_int(b) _Generic((b),            \
        databox: int_less_than_data,                    \
        default: def)                                   \


// Todo : implement equals for char*
#define eq(a, b) _Generic((a),                          \
        double:  eq_given_double(b),                    \
        int:     eq_given_int(b),                       \
        databox: eq_given_data(b),                      \
        default: def)((a),(b))

#define eq_given_double(b) _Generic((b),                \
        databox: double_eq_data,                        \
        double: double_eq_double,                       \
        default: def)

#define eq_given_data(b) _Generic((b),                  \
        databox: data_eq_data,                          \
        int:     data_eq_int,                           \
        double:  data_eq_double)

#define eq_given_int(b) _Generic((b),                   \
        databox: int_eq_data,                           \
        default: def)                                   \

#endif



