#include <stdlib.h>

// bindgen echo.h -o a.rs
typedef struct People {
    char* name;
    char* payload;
    unsigned char* content;
    int age;
} People;

extern People* Echo(People*);
