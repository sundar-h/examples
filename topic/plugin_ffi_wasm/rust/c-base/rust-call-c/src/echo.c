#include <stdio.h>

void echo_int(int i) {
    printf("received from rust: %d\n", i);
}

void echo_str(char* str) {
    printf("received from rust: %s\n", str);
}

char* return_str() {
    return "c string";
}

//void echo_bytes(int) {
//    printf("received from rust: %s\n", str);
//}
//
//void echo_struct(int) {
//}
