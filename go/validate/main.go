package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/google/gops/agent"
	"github.com/gorilla/mux"
	"github.com/gosuri/uitable"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

const (
	MB         = 2 ^ 20
	GB         = 2 ^ 30
	TestBucket = "test"
	MaxBytes   = 3 * GB
	addr       = "127.0.0.1:8000"
)

var client *minio.Client

func init() {
	var err error
	client, err = minio.New("localhost:9000", &minio.Options{
		Creds:  credentials.NewStaticV4("foo", "bar", ""),
		Secure: false,
	})
	panicIfError(err)
}

func main() {
	if err := agent.Listen(agent.Options{}); err != nil {
		log.Fatal(err)
	}
	time.Sleep(time.Hour)

	startServer(addr)
}

func startServer(addr string) {
	r := mux.NewRouter()
	r.HandleFunc("/", handleUploadFile).
		Methods(http.MethodPost)

	r.HandleFunc("/echo", handleEcho)

	svr := &http.Server{
		Addr:    addr,
		Handler: r,
		//ReadTimeout:       0,
		//ReadHeaderTimeout: 0,
		//WriteTimeout:      0,
		//IdleTimeout:       0,
		//MaxHeaderBytes:    0,
	}

	panicIfError(svr.ListenAndServe())
}

func handleUploadFile(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, MaxBytes)
	sum := sha256.New()
	// chunk copy
	rr := io.TeeReader(r.Body, sum)
	size, err := toS3(rr)
	if err != nil {
		panic(err)
	}
	sign := hex.EncodeToString(sum.Sum(nil))
	table := uitable.New()
	table.AddRow("Name", "Size", "Checksum")
	table.AddRow("Test", size, sign)

	fmt.Println(table)
}

func handleEcho(w http.ResponseWriter, r *http.Request) {
	fmt.Println("echo....")
	w.Write([]byte("Hello World"))
}

func toS3(r io.Reader) (int64, error) {
	info, err := client.PutObject(context.Background(), TestBucket, "test", r, -1, minio.PutObjectOptions{ContentType: "application/octet-stream"})
	if err != nil {
		return 0, err
	}
	return info.Size, nil
}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}
