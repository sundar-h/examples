#include "doubler.h"

extern const int FACTOR;
extern "C" {
    int doubler(int x) {
        return x * FACTOR;
    }
}