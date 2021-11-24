#include <stdio.h>
#include <stdlib.h>

void pass_int(int i) {
    printf("received from rust: %d\n", i);
}

void pass_str_owner(char* str) {
    printf("received from rust: %s\n", str);
//    free(str);
}

void pass_str_ref(char* str) {
    printf("received from rust: %s\n", str);
}

/**********************************************************
 * 返回 字符串字面量(字符串常量)
 **********************************************************/
//只能返回堆内存或者静态存储区内存
char* return_str_static() {
//     静态存储; 只读的.rodata段， 编译期
//     字符串常量所在的.rodata和.text（代码）段一样是由操作系统（loader）管理的，程序并不自行分配和销毁，也无权修改。
      return "I'm C String";
}

/**********************************************************
 * 返回 堆内存
 **********************************************************/
char* return_str_heap() {
    char *foo = malloc(sizeof(char) * 1024);
    snprintf(foo, 1024, "%s\n", "I'm C String"); /* puts "foo - bar\n" in foo */
    return foo;
}

/**********************************************************
 * 返回 栈内存 (栈再函数执行完即销毁,因此不能再函数之间传递, 函数没法返回栈上变量引用)
 **********************************************************/
char* return_str_stack() {
     char s[] = "I'm C String";
     printf("--------> In C: %s\n", s);
     return s;
}