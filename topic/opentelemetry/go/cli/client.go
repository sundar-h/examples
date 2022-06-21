package main

import (
	"context"
	"fmt"

	helloworld "github.com/example/telemetry/proto"

	"github.com/example/telemetry/lib"

	"github.com/example/telemetry/telemetry"
)

const (
	service = "demo-client"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	clean := telemetry.Init(service)
	defer clean(ctx)

	conn, err := lib.Dial("127.0.0.1:8101")
	lib.PanicIfErr(err)
	defer func() { _ = conn.Close() }()

	c := helloworld.NewInvokerClient(conn)
	callServer(c)
}

func callServer(c helloworld.InvokerClient) {
	// md := metadata.Pairs(
	// 	"timestamp", time.Now().Format(time.StampNano),
	// 	"client-id", "web-api-client-us-east-1",
	// 	"user-id", "some-test-user-id",
	// )
	//
	// ctx := metadata.NewOutgoingContext(context.Background(), md)
	response, err := c.Call(context.Background(), &helloworld.CallRequest{
		Name: "greeter",
	})
	lib.PanicIfErr(err)
	fmt.Println(response.GetMessage())
}
