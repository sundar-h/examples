#include "shared.h"

//int register_callback(Event* callback_target, rust_callback callback) {
int register_callback(void* callback_target, rust_callback callback) {
    cb_target = callback_target;
    cb = callback;

    return 1;
}

void trigger_callback() {
    cb(cb_target); // Will call callback(&rustObject, 7) in Rust.
}
