#include <stdio.h>
#include "library.h"

void library_function(int pin, LIBRARY_ENUM state) {
    printf("%i = %s\n", pin, state == LOW ? "LOW" : "HIGH");
}

extern void print_simple_pair(const simple_pair * const pair_ptr) {
    printf("{\n\ta=%i,\n\tb=%i\n}\n", pair_ptr->a, pair_ptr->b);
}
