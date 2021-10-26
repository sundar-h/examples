#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_____c_char Option_____c_char;

/**
 * [Rust FFI 实践](https://www.jianshu.com/p/f76631edbbfd)
 * [Rust functions that return allocated strings](http://jakegoulding.com/rust-ffi-omnibus/string_return/)
 * [CString](https://doc.rust-lang.org/std/ffi/struct.CString.html)
 * [CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html)
 *
 * 对象字符串 所有权再Rust和C之间的转移
 * 1. 首先，一个对象如果传递给了调用者，那么所有权会转移到调用者，这个是由 `Box::into_raw(Box::new(....))` 自动完成的。
 * 2. 其次，借出的对象一旦重新返回Rust,那么所有权就转移回Rust了，这个也是由 `*Box::from_raw(...)` 自动完成的。
 * 3. 如果想由调用者决定是否获取所有权和何时释放, 使用`&*tensor`避免获取所有权
 *
 *
 * ```
 * #include <stdio.h>
 * #include <stdint.h>
 *
 * extern char *
 * theme_song_generate(uint8_t length);
 *
 * extern void
 * theme_song_free(char *);
 *
 * int main(void) {
 *   char *song = theme_song_generate(5);
 *   printf("%s\n", song);
 *   theme_song_free(song);
 * }
 * // There’s not much interesting for the C version: the char * is returned, can be printed, and then is transferred back to be freed
 * ```
 *
 *
 *
 * CString: --> Into<Vec<u8>> 所有类型 (eg: String, &str)
 */
char *theme_song_generate(uint8_t length);

struct Option_____c_char theme_song_generate2(uint8_t length);

void theme_song_free(char *s);
