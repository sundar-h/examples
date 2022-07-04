package lib

import (
	"context"
	"log"
	"strings"

	"go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc"
	"google.golang.org/grpc"
)

func Dial(endpoint string) (*grpc.ClientConn, error) {
	return grpc.Dial(endpoint,
		grpc.WithInsecure(),
		grpc.WithBlock(),
		// grpc.WithUnaryInterceptor(otelgrpc.UnaryClientInterceptor()),
		grpc.WithChainUnaryInterceptor(otelgrpc.UnaryClientInterceptor()),
		grpc.WithStreamInterceptor(otelgrpc.StreamClientInterceptor()),
	)
}

// UnaryServerInterceptor 响应拦截器.
func UnaryServerInterceptor() grpc.UnaryServerInterceptor {
	return func(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
		log.Print("DialContext Request: ", GetMethod(info))

		return handler(ctx, req)
	}
}

func GetMethod(info *grpc.UnaryServerInfo) string {
	i := strings.LastIndex(info.FullMethod, "/")
	if i > 0 {
		// log.Info("DialContext Request: ", info.FullMethod[i+1:])
		return info.FullMethod[i+1:]
	}
	return info.FullMethod
}
