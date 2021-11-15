package sdk

type Source interface {
	Name() string
	Open() string
	Send() string
	Close() string
}
