#ifndef LIBGO_H
#define LIBGO_H

typedef void (*rust_callback)(int);

int c_trigger_callback(rust_callback callback);

#endif