package lib

import (
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
