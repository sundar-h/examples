#ifndef LIB_H
#define LIB_H

typedef void (*rust_callback)(int);
rust_callback cb;

int register_callback(rust_callback);

void trigger_callback();

#endif
