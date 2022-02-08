## 验证 Golang Http 文件传输接力过程中的内存变化
   1. server 服务器Post请求 接受上传文件
   2. 接收到的文件，实时上传到Minio，并通知计算Checksum
   3. 巩固掌握Golang PProf和trace 以及gops的使用

### 对照:
   1. TeeReader中转、接力
   2. 直接存放存放本地
   3. 直接计算Checksum然后丢弃
      #### 关注的点:
   1. 10G的文件，是否就需要 10G的内存 --> 已知的只是来说，不会的
   2. 这个过程中


### 通过Docker 实现内存限制
    1. docker 实例是否会crash掉

## 准备依赖

```
# minio
brew install minio/stable/minio
MINIO_ROOT_USER=admin MINIO_ROOT_PASSWORD=password minio server /mnt/data --console-address ":9001"
```
