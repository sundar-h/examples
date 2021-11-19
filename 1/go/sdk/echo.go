package sdk

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
//	Pad_cgo_0       [4]byte
//}

// go tool cgo -godefs sdk/echo.go
typedef struct People {
    char* name;
    char* payload;
    unsigned char* content;
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


type People struct {
	Name    string
	Payload string
	Content []byte
	Age     int
}


func fromCBytes(cBytes *C.uchar) []byte{
	return C.GoBytes(unsafe.Pointer(cBytes), C.len_of_uchar(cBytes))
}

func FromCPeople(p *C.struct_People) *People {
	return &People{
		Name:    C.GoString(p.name),
		Age:    int(p.age),
		Content: fromCBytes(p.content),
		Payload:    C.GoString(p.payload),
	}
}

func (p People) ToCPeople() *C.struct_People {
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
	Echo(*People)
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


	return People{
		Name:    "From_Go_Name",
		Payload: "From_go_Payload",
		Content: bytes.NewBufferString("From_Go_Content").Bytes(),
		Age:     666,
	}.ToCPeople()
}