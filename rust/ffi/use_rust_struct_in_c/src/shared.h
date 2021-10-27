//bindgen src/shared.h

//Rust 里面的结构体
typedef struct Event Event;
typedef void (*rust_callback)(*Event);

Event* cb_target;
rust_callback cb;


int register_callback(Event*, rust_callback);
void trigger_callback();
