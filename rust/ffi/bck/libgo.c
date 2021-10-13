#include "libgo.h"

int c_trigger_callback(rust_callback callback) {
    callback(7);
    return 1;
}
