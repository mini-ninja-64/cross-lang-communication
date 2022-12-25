#include <iostream>

#include <MyClass.hpp>
extern "C" {
#include <library.h>
}


int main (int argc, char *argv[]) {
    library_function(1, HIGH);

    MyClass myObject(0xFF);
    std::cout << myObject.getMyInt() << std::endl;
}
