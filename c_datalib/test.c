#include "databox.h"
#include "print.h"
#include <assert.h>
#include <stdio.h>

#define true 1
#define false 0
static databox one = { .data.num = 1, .type=NUM };
static databox two = { .data.num = 2.0, .type=NUM };
static databox three = { .data.num = 3.0, .type=NUM };
static databox hello = { .data.str = "Hello", .type=STR };
static databox box_42 = { .data.str = "42", .type=STR };

void test_inc() {
    printf("\n INCREMENT() TEST : \n");
    increment(&one);
    assert(one.data.num == 2.0);
    printf("%f", one.data.num);
}


void test_print() {
    printf("\n PRINT() TEST : \n");
    printf("Testing print() with databox :\n");
    print(two);
    printf("\n");
    printf("Testing print() with int : \n");
    print(1);
    printf("\n");
    printf("Testing print() with double : \n");
    print(1.1);
    printf("\n");
    printf("Testing print() with char* : \n");
    print("hello");
    printf("\n");
}

void test_new() {
    printf("\n NEW() TEST : \n");
    databox a = new_from_str("brand new bag");
    databox b = new_from_int(22);
    databox c = new_from_double(12.0);
    assert(a.data.str == "brand new bag");
    assert(a.type == STR);
    assert(b.type == NUM);
    assert(c.type == NUM);
}

void test_eq() {
    printf("\n EQ() TEST : \n");
    assert(eq(1.0, one) == true);
    assert(eq(two, two) == true);
    assert(eq(two, 3.0) == false);
    assert(eq(3.0, two) == false);
    assert(eq(1, one) == true);
    assert(eq(two, two) == true);
    assert(eq(two, 3) == false);
    assert(eq(3, two) == false);
}

void test_mul() {
    printf("\n MUL() TEST : \n");
    databox result = mul(three, two);
    assert(result.data.num==6.0);
}

void test_generic_gt() {
    printf("\n GREATER_THAN() TEST : \n");
    assert(gt(1.0, two) == false);
    assert(gt(two, 1.0) == true);
    assert(gt(two, 3.0) == false);
    assert(gt(3.0, two) == true);
    assert(gt(two, 1) == true);
    assert(gt(1, two) == false);
}

void test_generic_lt() {
    printf("\n LESS_THAN() TEST : \n");
    lt(two, 3);
    assert(lt(1.0, two) == true);
    assert(lt(two, 1.0) == false);
    assert(lt(two, 3.0) == true);
    assert(lt(3.0, two) == false);
    assert(lt(two, 3) == true);
    assert(lt(3, two) == false);
}

void test_add() {
    double number = 1;
    databox result = add(number, one);
    printf("testing databox + number: \n");
    assert(result.data.num);
    printf("result: %f + %f = %f Ok!\n", one.data.num, number, result.data.num);
}

void test_sub() {
    double number = 1;
    databox result = sub(number, one);
    printf("testing databox + number: \n");
    assert(result.data.num);
    printf("result: %f + %f = %f Ok!\n", one.data.num, number, result.data.num);
}

void test_greater_than() {
    int result = 42;

    result = data_greater_than_data(one, two);
    printf("testing data: %f > %f\n", one.data.num, two.data.num);
    assert(result == 0);

    result = data_greater_than_data(two, one);
    printf("testing data: %f > data: %f\n",two.data.num, one.data.num);
    assert(result == 1);

    printf("testing data: %s > data: %s\n",hello.data.str, box_42.data.str);
    result = data_greater_than_data(hello, box_42);
    assert(result == 1);

    printf("testing data: %s > data: %s\n",box_42.data.str, hello.data.str);
    result = data_greater_than_data(box_42, hello);
    assert(result == 0);
}

void test_dict() {
    dictionary *dict = dictionary_new();
    databox kevin = new_from_str("Eeasy peasy lemon squizzy");
    databox jason = new_from_str("le cheval c'est génial");
    dictionary_add(dict, "kevin", &kevin);
    dictionary_add(dict, "jason", &jason);
    printf("TESTING DICT : \n");
    print_data(*(databox*) dictionary_find(dict, "jason"));
}
int main() {
    test_generic_gt();
    test_generic_lt();
    test_greater_than();
    test_inc();
    test_add();
    test_sub();
    test_mul();
    test_print();
    test_dict();
    return 0;
}
