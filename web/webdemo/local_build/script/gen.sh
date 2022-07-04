#!/bin/bash

# yarn add ts-proto
# yarn install

rm -rf ./src/types/pb/*

# # Path to this plugin
PROTOC_GEN_TS_PATH="./node_modules/.bin/protoc-gen-ts_proto"

# # Directory to write generated code to (.js and .d.ts files)
OUT_DIR="./src/gen"

# PB_FILE=${HOME}/data/examples/topic/opentelemetry/go/proto/invoke.proto
PB_FILE=./vendor/proto/invoke.proto
GRPC_GATEWAY_PB=/Users/sanghaifa/go/bin/third_party/googleapis

protoc --plugin=${PROTOC_GEN_TS_PATH} \
    --ts_proto_opt=onlyTypes=true \
    --ts_proto_opt=context=true \
    --ts_proto_out=${OUT_DIR} ${PB_FILE} \
    -I./vendor/proto \
    -I${GRPC_GATEWAY_PB}
