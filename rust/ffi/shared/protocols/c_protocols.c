// bindgen protocol.c -o protocol.rs;
typedef struct Event {
    const char *topic;
    const char *body;
} Event;

typedef void (*RustCallback)(Event *e);
//RustCallback cb;
//
//struct Object;

int init(char* body);
const char* error();

//typedef struct Context {
//    RustCallback callback;
//
//    Name name;
//    Initialize init;
//    Finalize finalize;
//} Context;

//int register_callback(void* callback_target, RustCallback callback) {
//    cb_target = callback_target;
//    cb = callback;
//    return 1;
//}

//static inline int c_trigger_callback_struct(Event *e, RustCallback callback) {
//    Context cx;
//    cx.callback = callback;
//
//    callback(e);
//    return 1;
//}
