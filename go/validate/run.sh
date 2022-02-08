!#/bin/bash

mkdir /tmp/minio

MINIO_ROOT_USER=admin MINIO_ROOT_PASSWORD=admin minio server /tmp/minio -- console-address ":9001"


