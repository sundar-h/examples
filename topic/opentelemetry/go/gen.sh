#!/bin/bash

# if [ -z "$PROTOC_INSTALL" ]; then
# 	echo "PROTOC_INSTALL not set"
# 	exit 1
# fi

# basepath=$GOPATH/src
# pb_package=gitlab.sz.sensetime.com/viper/lite/lambda/function-manager-service/pb
# proto_install="$PROTOC_INSTALL"
# go_package=api
# rm -rf $go_package

# go get github.com/grpc-ecosystem/grpc-gateway@v1.16.0
# 查看grpc-gateway@v1.16.0 依赖得知, protoc-gen-go的commit为aa810b61a9
# go get @aa810b61a9 解析得到版本为 v1.2.0
# go get github.com/golang/protobuf@v1.2.0
# go get github.com/golang/protobuf@aa810b61a9
#  GO111MODULE=off sudo go install

# cp -r bin /usr/local/bin
# cp -r include /usr/local/include

protoc -I. \
 -I$GOPATH/pkg/mod/github.com/grpc-ecosystem/grpc-gateway@v1.16.0/third_party/googleapis \
--go_out=plugins=grpc:. ./proto/invoke.proto

protoc -I. \
 -I$GOPATH/pkg/mod/github.com/grpc-ecosystem/grpc-gateway@v1.16.0/third_party/googleapis \
 --grpc-gateway_out=logtostderr=true:. "./proto/invoke.proto"

# protoc: 3.5.1
# https://github.com/protocolbuffers/protobuf/releases?page=8
# https://github.com/protocolbuffers/protobuf/releases/download/v3.5.1/protoc-3.5.1-osx-x86_64.zip
# https://github.com/protocolbuffers/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip

# cd $basepath
# for i in $(ls $basepath/$pb_package/*.proto); do
# 	echo $i
# 	fn=$pb_package/$(basename "$i")
# 	$proto_install/bin/protoc -I$proto_install/include -I. \
# 		-I$GOPATH/src \
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis\
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway\
# 		--go_out=plugins=grpc:. "$fn"
# 	$proto_install/bin/protoc -I$proto_install/include -I. \
# 		-I$GOPATH/src \
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis\
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway\
# 		--grpc-gateway_out=logtostderr=true:. "$fn"
# 	$proto_install/bin/protoc -I$proto_install/include -I. \
# 		-I$GOPATH/src \
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis\
# 		-I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway\
# 		--swagger_out=logtostderr=true:. "$fn"
# done

# mv $pb_package/function-manager-service.pb.gw.go $(dirname $pb_package)/$go_package/searcher
