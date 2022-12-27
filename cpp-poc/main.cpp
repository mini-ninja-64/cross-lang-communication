extern "C" {
#include <library.h>
}

int main (int argc, char *argv[]) {
    library_function(1, HIGH);

    simple_pair pair = { 10, 20 };
    print_simple_pair(&pair);
}
