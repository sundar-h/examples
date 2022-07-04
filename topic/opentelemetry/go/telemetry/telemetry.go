package telemetry

import (
	"context"
	"log"
	"net/http"
	"time"

	"github.com/example/telemetry/lib"
	"github.com/pkg/errors"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/jaeger"
	"go.opentelemetry.io/otel/exporters/prometheus"
	"go.opentelemetry.io/otel/metric/global"
	"go.opentelemetry.io/otel/propagation"
	"go.opentelemetry.io/otel/sdk/metric/aggregator/histogram"
	controller "go.opentelemetry.io/otel/sdk/metric/controller/basic"
	"go.opentelemetry.io/otel/sdk/metric/export/aggregation"
	processor "go.opentelemetry.io/otel/sdk/metric/processor/basic"
	selector "go.opentelemetry.io/otel/sdk/metric/selector/simple"
	"go.opentelemetry.io/otel/sdk/resource"
	semconv "go.opentelemetry.io/otel/semconv/v1.4.0"
)

func Init(service string) func(ctx context.Context) {
	exp, err := jaeger.New(
		jaeger.WithAgentEndpoint(
			jaeger.WithAgentHost("localhost"),
			jaeger.WithAgentPort("6831"),
		),
	)
	lib.PanicIfErr(err)
	tp := sdktrace.NewTracerProvider(
		// Always be sure to batch in production.
		sdktrace.WithBatcher(exp),
		// Record information about this application in a Resource.
		sdktrace.WithResource(resource.NewWithAttributes(
			semconv.SchemaURL,
			semconv.ServiceNameKey.String(service),
			attribute.String("environment", "dev"),
			attribute.Int64("ID", 1),
		)),
	)
	otel.SetTracerProvider(tp)
	otel.SetTextMapPropagator(propagation.NewCompositeTextMapPropagator(propagation.TraceContext{}, propagation.Baggage{}))

	// Cleanly shutdown and flush telemetry when the application exits.
	return func(ctx context.Context) {
		// Do not make the application hang when it is shutdown.
		ctx, cancel := context.WithTimeout(ctx, time.Second*5)
		defer cancel()
		if err := tp.Shutdown(ctx); err != nil {
			log.Fatal(err)
		}
	}
}

// var (
// 	// 同步方式: 在关联上下文中调用 (span,Baggage)
// 	requestCounter         syncint64.Counter       // --> Add
// 	syncint64UpDownCounter syncint64.UpDownCounter // --> Add
// 	// 记录数据统计: 段时间范围内对数据进行采样(通常是请求持续时间或响应大小等)
// 	// 假设我们想监控某个应用在一段时间内的响应时间，
// 	// 最后监控到的样本的响应时间范围为 0s~10s。
// 	// 现在我们将样本的值域划分为不同的区间，
// 	// 即不同的 bucket，每个 bucket 的宽度是 0.2s。
// 	// 那么第一个 bucket 表示响应时间小于等于 0.2s 的请求数量，
// 	// 第二个 bucket 表示响应时间大于 0.2s 小于等于 0.4s 的请求数量，以此类推。
// 	// 可以聚合计算 平均请求时间
// 	syncint64Histogram syncint64.Histogram // --> Record
//
// 	syncfloat64Counter       syncfloat64.Counter       // --> Add
// 	syncfloat64UpDownCounter syncfloat64.UpDownCounter // --> Add
// 	syncfloat64Histogram     syncfloat64.Histogram     // --> Record
//
// 	// 异步接口只能在注册的callback中调用，才生效, 没有上下文
//
// 	// 只增
// 	asyncfloat64Counter asyncfloat64.Counter // -> Observe
// 	// 可增 可减
// 	asyncfloat64UpDownCounter asyncfloat64.UpDownCounter // --> Observe
// 	// 记录瞬时值(current value): 对求和无意思的数据(x需要平均值或者最大值,但不要求合), eg: 当前温度、内存使用量、线程数--
// 	asyncfloat64Gauge asyncfloat64.Gauge // --> Observe
// )

// func initMeter() {
// 	var err error
// 	meter := global.Meter("demo.example")
// 	requestCounter, err = meter.SyncInt64().Counter(
// 		"syncint64.Counter",
// 		instrument.WithDescription("只增数据 int64"),
// 		instrument.WithUnit("1"),
// 	)
// 	lib.PanicIfErr(err)
//
// 	syncint64UpDownCounter, err = meter.SyncInt64().UpDownCounter(
// 		"syncint64.UpDownCounter",
// 		instrument.WithDescription("可增可减 int64"),
// 		instrument.WithUnit("1"),
// 	)
// 	lib.PanicIfErr(err)
//
// 	syncint64Histogram, err = meter.SyncInt64().Histogram(
// 		"syncint64.Histogram",
// 		instrument.WithDescription("只增数据 int64"),
// 		instrument.WithUnit("1"),
// 	)
// 	lib.PanicIfErr(err)
//
// 	requestCounter, err = meter.SyncInt64().UpDownCounter(
// 		"syncint64.Counter",
// 		instrument.WithDescription("只增数据 int64"),
// 		instrument.WithUnit("1"),
// 	)
// 	lib.PanicIfErr(err)
// }

// NewPrometheusExporter Create the Prometheus exporter:
func NewPrometheusExporter() (http.Handler, error) {
	config := prometheus.Config{
		DefaultHistogramBoundaries: []float64{1, 2, 5, 10, 20, 50},
	}
	c := controller.New(
		processor.NewFactory(
			selector.NewWithHistogramDistribution(
				histogram.WithExplicitBoundaries(config.DefaultHistogramBoundaries),
			),
			aggregation.CumulativeTemporalitySelector(),
			processor.WithMemory(true),
		),
	)

	exp, err := prometheus.New(config, c)
	if err != nil {
		return nil, errors.New(err.Error())
	}

	global.SetMeterProvider(exp.MeterProvider())

	return exp, nil
}
