#!/bin/bash

protoc -I. --go_out=plugins=grpc:. helloworld/helloworld.proto
