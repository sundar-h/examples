#include <stdlib.h>

// bindgen echo.h -o a.rs
typedef struct People {
    char* name;
    unsigned char* content;
    char* payload;
    int age;
} People;

//pub struct People {
//    pub name: *mut ::std::os::raw::c_char,
//    pub content: *mut ::std::os::raw::c_uchar,
//    pub payload: *mut ::std::os::raw::c_char,
//    pub age: ::std::os::raw::c_int,
//    }
// }

extern People* Echo(People*);
