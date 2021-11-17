package sdk

import (
	"fmt"
)

/*
#include <stdlib.h>

typedef struct People {
    char* name;
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
 */
import "C"

type People struct {
	Name    string
	Payload string
	Age     int
}

func FromCPeople(p *C.struct_People) *People {
	return &People{
		Name:    C.GoString(p.name),
		Age:    int(p.age),
		Payload:    C.GoString(p.payload),
	}
}

func (p People) ToCPeople() *C.struct_People {
	return C.new_people()
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

	p2 := People{
		Name:    "LALALA",
		Payload: "KKKKK",
		Age:     66666,
	}.ToCPeople()

	return p2
}