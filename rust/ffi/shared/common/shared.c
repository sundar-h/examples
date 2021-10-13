//
// Created by 桑海发 on 2021/10/13.
//

typedef void (*rust_callback)(int);

static inline int c_trigger_callback_simple(rust_callback callback) {
    callback(7);
    return 1;
}

// ****************************************************************************************
struct Context {
    const char *name;
    int year;
};


static inline int c_trigger_callback_struct(struct Context *ctx, rust_callback callback) {
    callback(ctx->year);
    return 1;
}
