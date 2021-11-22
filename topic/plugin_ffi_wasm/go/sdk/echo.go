package sdk

// 查看中间代码: go tool cgo echo.go

import (
	"bytes"
	"fmt"
	"unsafe"
)

/*
#include <stdlib.h>
#include <string.h>

// C 定义转换 Go 定义
// go tool cgo -godefs sdk/echo.go
//type CPeople struct {
//	Name            *int8
//	Payload         *int8
//	Byte            *uint8
//	Age             int32
//	Pad_cgo_0       [4]byte // C和Golang结构体并不是一一对应的, 会有一些字节对齐的填充字段
//}

//extern struct GPeople gp;

// go tool cgo -godefs sdk/echo.go
typedef struct People {
    char* name;
    unsigned char* content;
    char* payload;
    int age;
} People;

static inline void free_people(People* p) {
    free(p->name);
    free(p->payload);
    free(p);
}

static inline People* new_people() {
	People *p;
	p = malloc(sizeof(*p));
	return p;
}

static inline int len_of_uchar(unsigned char* data) {
	return strlen((char*)data);
}
 */
import "C"


type GPeople struct {
	Name    string
	Content []byte
	Payload string
	Age     int
}

func (gp GPeople) String() string {
	return fmt.Sprintf("Name: %v\nContent: %v\nPayload: %v\nAge: %v", gp.Name, bytes.NewBuffer(gp.Content).String(), gp.Payload, gp.Age)
}

func fromCBytes(cBytes *C.uchar) []byte{
	return C.GoBytes(unsafe.Pointer(cBytes), C.len_of_uchar(cBytes))
}

func FromCPeople(p *C.struct_People) *GPeople {
	return &GPeople{
		Name:    C.GoString(p.name),
		Age:    int(p.age),
		Content: fromCBytes(p.content),
		Payload:    C.GoString(p.payload),
	}
}

//复制副本(到C语言空间)的方式
func (p GPeople) ToCPeople() *C.struct_People {
	//C.free(unsafe.Pointer(fr))
	// 哪里定义，哪里负责初始化
	p2 := C.new_people()
	p2.name = C.CString(p.Name)
	p2.age = C.int(p.Age)
	p2.content = (*C.uchar)(C.CBytes(p.Content))
	p2.payload = C.CString(p.Payload)
	return p2
}

type Echoer interface {
	Echo(*GPeople)
}

func Register(e Echoer) {
	instance = e
}

var instance Echoer

//export Echo
func Echo(p *C.struct_People) *C.struct_People {
	if instance == nil {
		fmt.Println("Echo Argument nil")
		return nil
	}
	defer C.free_people(p)

	instance.Echo(FromCPeople(p))

	return GPeople{
		Name:    "From_Go_Name",
		Payload: "From_go_Payload",
		Content: bytes.NewBufferString("From_Go_Content").Bytes(),
		Age:     666,
	}.ToCPeople()
}