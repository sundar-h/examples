//bindgen src/shared.h

typedef void (*rust_callback)(void*);
void* cb_target;
rust_callback cb;

int register_callback(void*, rust_callback);
void trigger_callback();
