#include <stdlib.h>

typedef struct People {
    char* name;
    char* payload;
    int age;
} People;

extern People* Echo(People*);
