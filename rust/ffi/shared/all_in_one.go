package main

// [CGO Document]()https://github.com/golang/go/wiki/cgo#function-variables)

/*
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
*/
import "C"
import "fmt"
// import gopointer "github.com/mattn/go-pointer"


// 不能直接通过go调用cb()
// 或者赋值 C.cb = cb
//export trigger_callback_simple
func trigger_callback_simple(cb C.rust_callback) {
    fmt.Println("Got trigger_callback_simple\n")
    C.c_trigger_callback_simple(cb)
}

//export trigger_callback_struct
func trigger_callback_struct(ctx *C.struct_Context, cb C.rust_callback) {
    fmt.Printf("Got trigger_callback_struct: %v\n", ctx)
    C.c_trigger_callback_struct(ctx, cb)
}

func main() {
}
