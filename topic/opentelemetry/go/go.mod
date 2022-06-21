module github.com/example/telemetry

go 1.15

require (
	github.com/golang/protobuf v1.5.2
	github.com/grpc-ecosystem/grpc-gateway v1.16.0
	go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc v0.32.0
	go.opentelemetry.io/otel v1.7.0
	go.opentelemetry.io/otel/exporters/jaeger v1.7.0
	go.opentelemetry.io/otel/sdk v1.7.0
	golang.org/x/net v0.0.0-20201021035429-f5854403a974
	google.golang.org/genproto v0.0.0-20200526211855-cb27e3aa2013
	google.golang.org/grpc v1.46.0
)
