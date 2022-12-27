typedef enum {
    HIGH,
    LOW,
} LIBRARY_ENUM;

typedef struct {
    int a;
    int b;
} simple_pair;

extern void library_function(int pin, LIBRARY_ENUM state);

extern void print_simple_pair(const simple_pair * const pair_ptr);
