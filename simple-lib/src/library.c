#include <stdio.h>
#include "library.h"

void library_function(int pin, LIBRARY_ENUM state) {
    printf("%i = %s\n", pin, state == LOW ? "LOW" : "HIGH");
}