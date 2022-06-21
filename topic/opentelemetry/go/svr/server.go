package main

import (
	"context"
	"fmt"
	"net"
	"time"

	"github.com/example/telemetry/lib"

	"github.com/example/telemetry/telemetry"

	"go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc"

	"go.opentelemetry.io/otel/trace"

	"go.opentelemetry.io/otel"

	"github.com/example/telemetry/helloworld"
	invoker "github.com/example/telemetry/proto"
	"go.opentelemetry.io/otel/attribute"
	"google.golang.org/grpc"
)

var (
	service = "demo-server"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	clean := telemetry.Init(service)
	defer clean(ctx)
	start()
}

type invokerImpl struct {
	conn   *grpc.ClientConn
	client helloworld.GreeterClient
}

func (impl *invokerImpl) Close() {
	_ = impl.conn.Close()
}

func (impl invokerImpl) Call(ctx context.Context, req *invoker.CallRequest) (*invoker.CallReply, error) {
	fmt.Println("received request: ", req.GetName())

	return &invoker.CallReply{
		Message: impl.call(ctx, req.GetName()),
	}, nil
}

func (impl invokerImpl) call(ctx context.Context, name string) string {
	tr := otel.Tracer("invoke call")
	_, span := tr.Start(ctx, "call",
		trace.WithAttributes(attribute.String("args", name))) //span.SetAttributes(attribute.Key("testset").String("value"))
	defer span.End()

	time.Sleep(100 * time.Millisecond)
	resp, err := impl.client.SayHello(ctx, &helloworld.HelloRequest{
		Name: name,
	})
	lib.PanicIfErr(err)
	return resp.GetMessage()
}

func start() {
	fmt.Println("listen :8101")
	lis, err := net.Listen("tcp", ":8101")
	lib.PanicIfErr(err)

	s := grpc.NewServer(
		//grpc.WithInsecure(),
		grpc.UnaryInterceptor(otelgrpc.UnaryServerInterceptor()),
		grpc.StreamInterceptor(otelgrpc.StreamServerInterceptor()),
	)

	println("dial rust...")
	conn, err := lib.Dial("127.0.0.1:8102")
	lib.PanicIfErr(err)
	client := helloworld.NewGreeterClient(conn)

	println("serve...")
	invoker.RegisterInvokerServer(s, &invokerImpl{conn: conn, client: client})
	err = s.Serve(lis)
	lib.PanicIfErr(err)
}
